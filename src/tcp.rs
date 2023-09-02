use std::io::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use crate::irc::irc_commands::IRCCommands;
use crate::irc::irc_context::IRCContext;
use crate::irc::irc_message_handler::IRCMessageHandler;
use crate::irc::irc_message_parsed::IRCMessageParsed;

async fn wait_for_command(send_rx: Arc<Mutex<mpsc::Receiver<String>>>) -> Option<String> {
	send_rx.lock().await.recv().await
}

async fn wait_for_connection(conn_rx: Arc<Mutex<mpsc::Receiver<String>>>) -> Option<TcpStream> {
	//println!("[DEBUG] Waiting for connection order...");
	let addr = conn_rx.lock().await.recv().await;
	//println!("[DEBUG] Got somewhere to connect to");
	match addr {
		// TODO handle error
		Some(addr) => {
			if addr == "disconnect" {
				println!("Disconnected from server");
				return None;
			}
			let ret = TcpStream::connect(addr).await.ok();
			println!("Connected");
			ret
		},
		None => None
	}
}

async fn handle_command(tcp_stream: &mut TcpStream, command: Option<String>) {
	//println!("[DEBUG] Got message to send");
	if let Some(command) = command {
		//println!("[DEBUG] Sent message ");
		// TODO handle error
		tcp_stream.write(command.as_bytes()).await.ok();
	}
}

async fn handle_irc_message(raw_message: String, tcp_stream: &mut TcpStream) {
	let raw_message = raw_message
		.trim_start_matches('\0')
		.trim_end_matches('\0').to_string();
	let message = IRCMessageParsed::parse(raw_message);
	let command = match IRCCommands::new(message.command.as_str()) {
		Ok(val) => val,
		Err(e) => {
			//eprintln!("[DEBUG] {}", e);
			IRCCommands::Unknown
		}
	};

	command.format(message.clone());
	if let IRCCommands::Ping = command {
		let code: String = format!(":{}", message.data.trim());

		if let Ok(message) = IRCCommands::Pong.craft(
			"", code.as_str(), IRCContext::new()
		) {
			if let Err(e) = tcp_stream.write(
				format!("{}\r\n", message.as_raw()).as_bytes()
			).await {
				eprintln!("Error while sending pong: {}", e)
			}
			println!("-> {}", message.as_raw());
		} else {
			eprintln!("Error can't send pong")
		}
	}

	println!("{}", message.as_formatted());
}


async fn handle_message(
	buf: [u8; 4096],
	len: Result<usize, Error>,
	memory: &mut Vec<u8>,
	tcp_stream: &mut TcpStream
) -> bool {
	match len {
		Ok(n) if n == 0 => {
			println!("Disconnected from server");
			return false
		},
		Err(err) => {
			eprintln!("Error with server: {}", err);
			return false
		},
		_ => {}
	}
	let received_data = buf.to_vec();
	//println!("[DEBUG] Raw data <{}>", String::from_utf8_lossy(&received_data));
	//println!("[DEBUG] In memory before join <{}>", String::from_utf8_lossy(memory));
	memory.extend_from_slice(&received_data);
	//println!("[DEBUG] In memory <{}>", String::from_utf8_lossy(memory));
	while let Some(split_idx) = memory
		.windows(2)
		.position(|window| window == b"\r\n") {
		let message_bytes: Vec<u8>  = memory.drain(..=split_idx + 1).collect();
		let message = String::from_utf8_lossy(&message_bytes).to_string();
		//println!("[DEBUG] Drained from memory <{}>", message);
		handle_irc_message(message, tcp_stream).await;
	}

	true
}

pub fn start_poll_thread(
	send_rx: Arc<Mutex<mpsc::Receiver<String>>>,
	conn_rx: Arc<Mutex<mpsc::Receiver<String>>>
) -> JoinHandle<()> {
	tokio::task::spawn(async move {
		let mut stream: Option<TcpStream> = None;
		let mut memory: Vec<u8> = Vec::new();
		loop {
			let send_rx = send_rx.clone();
			let conn_rx = conn_rx.clone();
			// TODO is this level of indent necessary ?
			//println!("[DEBUG] Polling");
			let mut buf: [u8; 4096] = [0; 4096];
			match stream.as_mut() {
				None => {
					stream = wait_for_connection(conn_rx).await;
				},
				Some(tcp_stream) => {
					//println!("[DEBUG] Connected, waiting for action");
					select! {
						ret = wait_for_connection(conn_rx) => {
							stream = ret;
						}
						command = wait_for_command(send_rx) => {
							handle_command(tcp_stream, command).await
						},
						n = tcp_stream.read(&mut buf) => {
							if !handle_message(buf, n, &mut memory, tcp_stream).await {
								stream = None
							}
						}
					}
				}
			}
		}
	})
}