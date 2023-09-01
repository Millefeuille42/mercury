mod prompt_commands;
mod utils;
mod tcp;

use utils::irc_comm_channels::IRCCommChannels;
use utils::irc_comm_channels::spawn_channel;
use tcp::start_poll_thread;

use std::io;
use std::sync::Arc;
use tokio::select;
use tokio::sync::Mutex;

async fn command_manager(command: String, channels: IRCCommChannels<'_>) {
	let mut command = command.trim();
	if !command.starts_with('/') {
		if let Err(err) = channels.write(command).await {
			eprintln!("{}", err)
		}
		return;
	}
	command = command.trim_matches('/');

	let args: Vec<&str> = command.split_whitespace().collect();

	if let Err(err) = prompt_commands::commands::execute(args, channels).await {
		eprintln!("{}", err)
	}
}

async fn prompt_user() -> String {
	//println!("[DEBUG] Prompting");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");

	return input.trim().to_string();
}

#[tokio::main]
async fn main() {
	let (send_tx, send_rx) = spawn_channel();
	let (conn_tx, conn_rx) = spawn_channel();
	let send_rx = Arc::new(Mutex::new(send_rx));
	let conn_rx = Arc::new(Mutex::new(conn_rx));

	let mut poll_handle = start_poll_thread(send_rx, conn_rx);
	loop {
		let rx_channels = IRCCommChannels::new(
			&send_tx,
			&conn_tx
		);
		select! {
			_ = &mut poll_handle => {
				//println!("[DEBUG] Got in poll branch");
			},
			input = prompt_user() => {
				//println!("[DEBUG] Got in prompt branch");
				command_manager(input, rx_channels).await;
			}
		}
	}
}