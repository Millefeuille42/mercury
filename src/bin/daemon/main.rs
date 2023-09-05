mod irc_connection;

use irc_connection::IRCConnection;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use mercury::irc;
use mercury::buffer::Buffer;
use mercury::irc::MessageHandler;
use mercury::thread_pool::ThreadPool;

use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;


fn poll_read_servers(connections: &mut [IRCConnection], message_tx: &Sender<String>) {
	for connection in connections {
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
				if let Err(e) = message_tx.send(
					format!("{}\r\n", message.as_raw())
				) {
					eprintln!("Error while sending message to workers: {}", e);
					continue;
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

fn server_loop(command_rx: &mpsc::Receiver<String>, message_tx: &Sender<String>) {
	let mut connections: Vec<IRCConnection> = Vec::new();
	connections.push(IRCConnection::new("chat.freenode.net:6667"));
	if let Err(e) = connections[0].connect() {
		match e.kind() {
			std::io::ErrorKind::ConnectionRefused => (),
			_ => eprintln!("{}", e.kind())
		}
	}

	// TODO wait for connect command to create a connection
	// TODO associate info to connection (id ?)
	// TODO give an id to connections to route incoming messages
	// TODO save the messages to log files (rotation ? per_id ?)
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
					mpsc::TryRecvError::Empty => (),
					_ => eprintln!("{}", e)
				}
			}
		}
		poll_read_servers(&mut connections, message_tx)
	}
}

fn main() {
	// TODO Read config / arguments
	//  from config: bind_address, max_workers, timeout, writing_interface

	let (command_tx, command_rx) = mpsc::channel::<String>();
	let (message_tx, message_rx) = broadcast::channel::<String>(8);

	thread::spawn( move || {
		let pool = ThreadPool::new(4).expect("Couldn't start thread pool");
		let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
		for stream in listener.incoming() {
			let stream = stream.unwrap();
			let tx = command_tx.clone();
			let rx = message_rx.resubscribe();
			pool.execute( move || {
				handle_connection(stream, tx, rx);
			});
		}
	});

	server_loop(&command_rx, &message_tx);
}

fn handle_connection(mut stream: TcpStream, tx: mpsc::Sender<String>, mut rx: broadcast::Receiver<String>) {
	stream.set_read_timeout(Some(Duration::from_secs(60))).ok();
	stream.set_nonblocking(true).ok();

	let mut memory = Buffer::new();
	loop {
		match rx.try_recv() {
			Ok(message) => {
				if let Err(e) = stream.write_all(message.as_bytes()) {
					eprintln!("Error while sending message to client: {}", e)
				}
			},
			Err(e) => {
				match e {
					broadcast::error::TryRecvError::Empty => (),
					_ => eprintln!("Error while receiving message from main {}", e)
				}
			}
		}

		let mut buf: [u8; 1024] = [0; 1024];
		match stream.read(&mut buf) {
			Err(e) => {
				match e.kind() {
					std::io::ErrorKind::WouldBlock => (),
					_ => {
						eprintln!("Error on read: {}", e);
						break;
					}
				}
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