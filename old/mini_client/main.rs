use std::{env, io, thread};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use prost::Message;
use mercury::buffer::Buffer;
use mercury::proto;

fn prompt_user() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");

	return input.trim().to_string();
}

fn main() -> Result<(), io::Error> {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		eprintln!("Usage: mini_client <host> <port>");
		return Err(io::Error::from(io::ErrorKind::InvalidInput))
	}

	let mut stream: TcpStream = TcpStream::connect(
		format!("{}:{}", args[1], args[2])
	)?;
	stream.set_nonblocking(true)?;
	let (prompt_tx, prompt_rx) = mpsc::channel::<String>();

	thread::spawn(|| {
		let prompt_tx = prompt_tx;
		loop {
			prompt_tx.send(prompt_user()).expect("Unable to send to main");
		}
	});

	let mut memory: Buffer = Buffer::new();
	loop {
		let mut buf: [u8; 1024] = [0; 1024];
		match stream.read(&mut buf) {
			Err(e) => {
				match e.kind() {
					io::ErrorKind::WouldBlock => (),
					_ => {
						eprintln!("Error on read from server: {}", e);
						break;
					}
				}
			},
			Ok(n) => {
				if n == 0 {
					eprintln!("Server disconnected");
					break;
				}
				memory.save(&buf);
				while let Some(message) = memory.next(2, b"\r\n") {
					match proto::Command::decode(message.trim().as_bytes()) {
						Ok(message) => println!("{}", format!("{}{}{}",
							message.command,
							message.target,
							message.parameters
						)),
						Err(e) => eprintln!("Failed to decode message from server: {}", e)
					}
				}
			}
		}
		match prompt_rx.try_recv() {
			Ok(message) => {
				let mut encoded_message = Vec::new();
				let encoding = proto::Command{
					command: "SEND".to_string(),
					target: "*".to_string(),
					parameters: message.trim().to_string(),
				}.encode(&mut encoded_message);
				match encoding {
					Ok(_) => {
						let a = proto::Command::decode(encoded_message.as_slice()).expect("it crashed");
						encoded_message.push(b'\r');
						encoded_message.push(b'\n');
						if let Err(e) = stream.write_all(&encoded_message) {
							eprintln!("Error while sending message to server: {}", e)
						}
					}
					Err(e) => {
						eprintln!("Error while encoding message to server: {}", e)
					}
				}
			}
			Err(e) => {
				match e {
					mpsc::TryRecvError::Empty => (),
					_ => eprintln!("Error while receiving message from prompt {}", e)
				}
			}
		}
	}

	Ok(())
}