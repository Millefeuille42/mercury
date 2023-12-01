mod irc_server;

use std::sync::mpsc;
use std::thread;
use irc_server::IRCServer;
use mercury::irc;
use mercury::irc::MessageHandler;

fn poll_servers(servers: &mut [IRCServer]) {
	for server in servers {
		if !server.is_connected() {
			if let Err(e) = server.connect() {
				match e.kind() {
					std::io::ErrorKind::UnexpectedEof => (),
					_ => eprintln!("{}", e.kind())
				}
				continue;
			}
		}

		let message = server.try_read();
		if let Err(e) = message {
			match e.kind() {
				std::io::ErrorKind::WouldBlock => (

				),
				std::io::ErrorKind::BrokenPipe => {
					eprintln!("read 0");
				},
				_ => eprintln!("{}", e)
			}
			continue;
		}
		let message = irc::Message::parse(message.unwrap());
		let command = match irc::Commands::new(message.command.as_str()) {
			Ok(val) => val,
			Err(_) => irc::Commands::Unknown
		};
		command.format(message.clone());

		if let irc::Commands::Ping = command {
			let code: String = format!(":{}", message.data.trim());

			if let Ok(message) = irc::Commands::Pong.craft(
				"", code.as_str(), irc::Context::new()
			) {
				if let Err(e) = server.write(
					format!("{}\r\n", message.as_raw()).as_bytes()
				) {
					eprintln!("Error while sending pong: {}", e);
					continue;
				}
				println!("-> {}", message.as_raw());
			} else { eprintln!("Error can't send pong") }

			continue;
		}
		println!("{}", message.as_formatted());
	}
}

fn poll_command(servers: &mut [IRCServer], rx: &mpsc::Receiver<String>) {
	match rx.try_recv() {
		Ok(message) => {
			// TODO use message data for commands and routing
			for server in servers {
				if server.is_connected() {
					server.write(message.as_bytes()).expect("Unable to write to server");
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
}

fn prompt_user(_: irc::Context) -> String {
	let mut input = String::new();
	// TODO to put when using ncurses or such
	// print!("{}{}",
	// 	if ctx.nick.is_empty() { "".to_string() } else {format!("{}@", ctx.nick)},
	// 	ctx.channel
	// );
	// io::stdout().flush().expect("Failed to print prompt");
	std::io::stdin().read_line(&mut input).expect("Failed to read line");

	return input.trim().to_string();
}

fn main() {
	let mut servers: Vec<IRCServer> = Vec::new();
	let (mut tx, rx) = mpsc::channel::<String>();

	//servers.push(IRCServer::new("localhost:8888"));
	servers.push(IRCServer::new("irc.oftc.net:6667"));

	thread::spawn(move || {
		loop {
			poll_servers(&mut servers);
			poll_command(&mut servers, &rx);
		}
	});

	let mut ctx = irc::Context::new();
	loop {
		let command = prompt_user(ctx.clone());
		let command = command.trim();

		if !command.starts_with('/') {
			//let message = irc::Message::craft(
			//	"PRIVMSG", command, ctx.clone()
			//);
			//if let Err(err) = message {
			//	eprintln!("{}", err);
			//	continue;
			//}
			let message = command
				.trim_start_matches('\0')
				.trim_end_matches('\0')
				.trim_start_matches('\n')
				.trim_end_matches('\n')
				.trim_start_matches('\r')
				.trim_end_matches('\r').to_string() + "\r\n"
				;

			if let Err(err) = tx.send(message) {
				eprintln!("{}", err)
			}
			continue
		}
		//let command = command.trim_matches('/');
		//let args: Vec<&str> = command.split_whitespace().collect();
		// TODO prevent sending messages or such if user is not connected
	}
}