mod irc_connection;

use irc_connection::IRCConnection;
use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::{thread};
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::time::{Duration};
use mercury::buffer::Buffer;
use mercury::irc;
use mercury::irc::MessageHandler;
use mercury::thread_pool::ThreadPool;

fn server_loop(command_rx: &mpsc::Receiver<String>) {
	let mut connections: Vec<IRCConnection> = Vec::new();
	connections.push(IRCConnection::new("chat.freenode.net:6667"));
	if let Err(e) = connections[0].connect() {
		match e.kind() {
			std::io::ErrorKind::ConnectionRefused => (),
			_ => eprintln!("{}", e.kind())
		}
	}

	// TODO wait for connect command to create a stream
	// TODO associate context to connection
	// TODO give and id to connections to route incoming and outgoing messages
	// TODO send the messages to connected clients / log file
	// TODO factorize this in multiple functions
	loop {
		match command_rx.try_recv() {
			Ok(message) => {
				println!("Sending: {}", message);
				for connection in &mut connections {
					if connection.is_connected() {
						connection.write(message.as_bytes()).expect("Unable to write to server");
					}
				}
			},
			Err(e) => {
				match e {
					TryRecvError::Empty => (),
					_ => eprintln!("{}", e)
				}
			}
		}
		for connection in &mut connections {
			if !connection.is_connected() {
				if let Err(e) = connection.connect() {
					match e.kind() {
						std::io::ErrorKind::ConnectionRefused => (),
						_ => eprintln!("{}", e.kind())
					}
					continue;
				}
			}
			match connection.try_read() {
				Ok(message) => {
					let message = irc::Message::parse(message);
					let command = match irc::Commands::new(message.command.as_str()) {
						Ok(val) => val,
						Err(_) => {
							//eprintln!("[DEBUG] {}", e);
							irc::Commands::Unknown
						}
					};

					command.format(message.clone());
					if let irc::Commands::Ping = command {
						let code: String = format!(":{}", message.data.trim());

						if let Ok(message) = irc::Commands::Pong.craft(
							"", code.as_str(), irc::Context::new()
						) {
							if let Err(e) = connection.write(
								format!("{}\r\n", message.as_raw()).as_bytes()
							) {
								eprintln!("Error while sending pong: {}", e)
							}
							println!("-> {}", message.as_raw());
						} else {
							eprintln!("Error can't send pong")
						}
					}

					println!("{}", message.as_formatted());
				}
				Err(e) => {
					match e.kind() {
						std::io::ErrorKind::WouldBlock => (),
						_ => eprintln!("{}", e)
					}
				}
			}
		}
	}
}

fn main() {
	// TODO Read config / arguments
	//  from config: bind_address, max_workers, timeout, writing_interface

	let (tx, command_rx) = mpsc::channel::<String>();
	thread::spawn( move || {
		let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
		let pool = ThreadPool::new(4).expect("Couldn't start thread pool");
		for stream in listener.incoming() {
			let stream = stream.unwrap();
			let tx = tx.clone();
			pool.execute( move || {
				handle_connection(stream, tx);
			});
		}
	});

	server_loop(&command_rx);
}

fn handle_connection(mut stream: TcpStream, tx: mpsc::Sender<String>) {
	stream.set_read_timeout(Some(Duration::from_secs(60))).ok();
	let mut memory = Buffer::new();
	loop {
		let mut buf: [u8; 1024] = [0; 1024];
		match stream.read(&mut buf) {
			Err(e) => {
				eprintln!("Error on read: {}", e);
				break;
			},
			Ok(n) => {
				if n == 0 {
					eprintln!("Client disconnected");
					break;
				}
				memory.save(&buf);
				while let Some(message) = memory.next(2, b"\r\n") {
					if let Err(e) = tx.send(format!("{}\r\n", message.trim())) {
						eprintln!("Couldn't write to channel: {}", e)
					}
				}
			}
		}
	}
}