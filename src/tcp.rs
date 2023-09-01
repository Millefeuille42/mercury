use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use crate::irc::irc_context::IRCContext;
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

async fn handle_message(buf: [u8; 1024], len: Result<usize, std::io::Error>) -> Option<String> {
	//println!("[DEBUG] Got data from server");
	// TODO return Result instead of Option
	match len {
		Ok(n) if n == 0 => {
			println!("Disconnected from server");
			None
		},
		Ok(_) => {
			let message = String::from_utf8_lossy(&buf).to_string();
			println!("{}", message);
			Some(message)
		}
		Err(err) => {
			eprintln!("Error with server: {}", err);
			None
		},
	}
}

async fn handle_irc_message(message: String, tcp_stream: &mut TcpStream) {
	// TODO do proper handling with matches and stuff
	if message.starts_with("PING") {
		if let Some(colon_idx) = message.find(':') {
			let code: &str = message[colon_idx + 1..].trim();
			let message = IRCMessageParsed::craft(
				"PONG", code, IRCContext::new()
			).ok().unwrap();
			// TODO handle error
			tcp_stream.write(message.as_raw().as_bytes()).await.ok();
			println!("-> {}", message.as_raw());
		}
	}
}

pub fn start_poll_thread(
	send_rx: Arc<Mutex<mpsc::Receiver<String>>>,
	conn_rx: Arc<Mutex<mpsc::Receiver<String>>>
) -> JoinHandle<()> {
	tokio::task::spawn(async move {
		let mut stream: Option<TcpStream> = None;
		loop {
			let send_rx = send_rx.clone();
			let conn_rx = conn_rx.clone();
			// TODO is this level of indent necessary ?
			//println!("[DEBUG] Polling");
			let mut buf: [u8; 1024] = [0; 1024];
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
							let message = handle_message(buf, n).await;
							match message {
								Some(message) => handle_irc_message(message, tcp_stream).await,
								None => {stream = None}
							};
						}
					}
				}
			}
		}
	})
}