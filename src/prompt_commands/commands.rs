use std::error::Error;
use crate::prompt_commands::connect::connect;
use crate::prompt_commands::quit::quit;
use crate::irc::irc_context::IRCContext;
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

pub async fn execute<'a>(args: Vec<&str>, channels: IRCCommChannels<'a>, ctx: &mut IRCContext) -> Result<(), Box<dyn Error>> {
	if let None = args.first() {
		return Err("command not found!".to_string().into())
	}

	Commands::new(args.first().unwrap()).execute(args, channels, ctx).await
}