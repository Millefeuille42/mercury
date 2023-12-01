use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, SystemTime};
use mercury::buffer::Buffer;

pub struct IRCServer {
	address: String,
	memory: Buffer,
	stream: Option<TcpStream>,
	retry_after: SystemTime
}

impl IRCServer {
	pub fn new(address: &str) -> Self {
		IRCServer {
			address: address.to_string(),
			memory: Buffer::new(),
			stream: None,
			retry_after: SystemTime::now()
		}
	}

	pub fn connect(&mut self) -> Result<(), std::io::Error> {
		if SystemTime::now() < self.retry_after {
			return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))
		}

		let result = TcpStream::connect(self.address.to_string());
		match result {
			Err(e) => {
				match e.kind() {
					std::io::ErrorKind::UnexpectedEof => {
						self.retry_after = SystemTime::now() + Duration::from_secs(5);
						Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof))
					},
					_ => Err(e)
				}
			}
			Ok(stream) => {
				self.stream = Some(stream);
				self.stream.as_mut().unwrap().set_nonblocking(true)
			}
		}
	}

	pub fn try_read(&mut self) -> Result<String, std::io::Error> {
		if let Some(message) = self.memory.next(2, b"\r\n") {
			return Ok(message)
		}

		let mut buf: [u8; 1024] = [0; 1024];
		match self.stream.as_mut() {
			None => Err(std::io::Error::from(std::io::ErrorKind::NotConnected)),
			Some(stream) => {
				match stream.read(&mut buf) {
					Err(e) => Err(e),
					Ok(n) => {
						if n == 0 {
							self.stream = None;
							return Err(std::io::Error::from(
								std::io::ErrorKind::BrokenPipe
							));
						}
						self.memory.save(&buf);
						if let Some(message) = self.memory.next(2, b"\r\n") {
							return Ok(message)
						}
						Err(std::io::Error::from(std::io::ErrorKind::WouldBlock))
					}
				}
			}
		}
	}

	pub fn write(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
		match self.stream.as_mut() {
			None => Err(std::io::Error::from(std::io::ErrorKind::NotConnected)),
			Some(stream) => {
				stream.write_all(buf)
			}
		}
	}

	pub fn is_connected(&self) -> bool {
		self.stream.is_some()
	}
}