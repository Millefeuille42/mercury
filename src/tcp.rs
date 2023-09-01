use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

async fn wait_for_command(send_rx: Arc<Mutex<mpsc::Receiver<String>>>) -> Option<String> {
	send_rx.lock().await.recv().await
}

async fn wait_for_connection(conn_rx: Arc<Mutex<mpsc::Receiver<String>>>) -> Option<TcpStream> {
	//println!("[DEBUG] Waiting for connection order...");
	let addr = conn_rx.lock().await.recv().await;
	//println!("[DEBUG] Got somewhere to connect to");
	match addr {
		// TODO handle error
		Some(addr) => TcpStream::connect(addr).await.ok(),
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

async fn handle_message(buf: [u8; 1024], len: Result<usize, std::io::Error>) -> bool {
	//println!("[DEBUG] Got data from server");
	// TODO return result instead of bool
	match len {
		Ok(n) if n == 0 => {
			println!("Disconnected from server...");
			false
		},
		Ok(_) => {
			let message = String::from_utf8_lossy(&buf).to_string();
			println!("{}", message);
			true
		}
		Err(err) => {
			eprintln!("Error with server: {}", err);
			false
		},
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
			// TODO Is this level of indent necessary ?
			//println!("[DEBUG] Polling");
			let mut buf: [u8; 1024] = [0; 1024];
			match stream.as_mut() {
				None => {
					stream = wait_for_connection(conn_rx).await;
				},
				Some(tcp_stream) => {
					//println!("[DEBUG] Connected, waiting for action");
					select! {
						command = wait_for_command(send_rx) => {
							handle_command(tcp_stream, command).await
						},
						n = tcp_stream.read(&mut buf) => {
							if !handle_message(buf, n).await {
								stream = None;
							}
						}
					}
				}
			}
		}
	})
}