use std::error::Error;
use crate::prompt_commands::connect::connect;
use crate::prompt_commands::quit::quit;
use crate::utils::irc_comm_channels::IRCCommChannels;

enum Commands {
	Connect,
	Quit
}

impl Commands {
	async fn execute<'a>(&self, args: Vec<&str>, channels: IRCCommChannels<'a>) -> Result<(), Box<dyn Error>> {
		match self {
			Commands::Connect => connect(args, channels).await,
			Commands::Quit => quit(args, channels).await
		}
	}
}

pub async fn execute<'a>(args: Vec<&str>, channels: IRCCommChannels<'a>) -> Result<(), Box<dyn Error>> {
	match args[0] {
		"connect" => Commands::Connect.execute(args, channels).await,
		"quit" => Commands::Quit.execute(args, channels).await,
		&_ => Err("command not found!".to_string().into())
	}
}