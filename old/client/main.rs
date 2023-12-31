mod prompt_commands;
mod utils;
mod tcp;

use mercury::irc;

use utils::irc_comm_channels::IRCCommChannels;
use utils::irc_comm_channels::spawn_channel;
use tcp::start_poll_thread;

use std::io;
use std::sync::Arc;
use tokio::select;
use tokio::sync::Mutex;

async fn command_manager(command: String, channels: IRCCommChannels<'_>, ctx: &mut irc::Context) {
	let mut command = command.trim();
	if !command.starts_with('/') {
		let ctx_ = ctx.clone();
		let message = irc::Message::craft("PRIVMSG", command, ctx_).ok().unwrap();
		if let Err(err) = channels.write(message.as_raw().as_str()).await {
			eprintln!("{}", err)
		}
		return;
	}
	command = command.trim_matches('/');

	let args: Vec<&str> = command.split_whitespace().collect();

	// TODO prevent sending messages or such if user is not connected
	if let Err(err) = prompt_commands::commands::execute(args, channels, ctx).await {
		eprintln!("{}", err)
	}
}

async fn prompt_user(_: irc::Context) -> String {
	//println!("[DEBUG] Prompting");
	let mut input = String::new();
	// TODO to put when using ncurses or such
	// print!("{}{}",
	// 	if ctx.nick.is_empty() { "".to_string() } else {format!("{}@", ctx.nick)},
	// 	ctx.channel
	// );
	// io::stdout().flush().expect("Failed to print prompt");
	io::stdin().read_line(&mut input).expect("Failed to read line");

	return input.trim().to_string();
}

#[tokio::main]
async fn main() {
	let (send_tx, send_rx) = spawn_channel();
	let (conn_tx, conn_rx) = spawn_channel();
	let send_rx = Arc::new(Mutex::new(send_rx));
	let conn_rx = Arc::new(Mutex::new(conn_rx));
	let mut context = irc::Context::new();

	let mut poll_handle = start_poll_thread(send_rx, conn_rx);
	loop {
		let rx_channels = IRCCommChannels::new(
			&send_tx,
			&conn_tx
		);
		let ctx_ = context.clone();
		select! {
			_ = &mut poll_handle => {
				//println!("[DEBUG] Got in poll branch");
			},
			input = prompt_user(ctx_) => {
				//println!("[DEBUG] Got in prompt branch");
				command_manager(input, rx_channels, &mut context).await;
			}
		}
	}
}