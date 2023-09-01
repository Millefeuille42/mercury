use std::error::Error;
use crate::prompt_commands::connect::connect;
use crate::prompt_commands::quit::quit;
use crate::prompt_commands::part::part;
use crate::prompt_commands::query::query;
use crate::prompt_commands::send::send;
use crate::prompt_commands::me::me;
use crate::irc::irc_context::IRCContext;

use crate::utils::irc_comm_channels::IRCCommChannels;

enum Commands {
	Connect,	// Connect to server
	Quit,		// Quit server
	Join,		// Join channel
	Part,		// Leave channel
	In,			// Set foreground channel
	Home,		// Remove foreground channel
	Send,		// Send message to provided channel
	Query,		// Send message to user
	Me,			// Send action message to foreground channel (or to provided one)
	Nick,		// Change nick
	Away,		// Set / Unset away status
	Back,		// Unset away status
	Whois,		// Get information on user
	List,		// List channels
	Names,		// List users in foreground channel (or in provided one)
	Topic,		// Get foreground channel (or provided one) topic
	MotD,		// Get MotD
	Ping,		// Check server responsiveness
	Raw,		// Send raw IRC message
	Help,		// Print help
	Unknown,
}

impl Commands {
	async fn execute<'a>(&self, args: Vec<&str>, channels: IRCCommChannels<'a>) -> Result<(), Box<dyn Error>> {
	fn new(command: &str) -> Self {
		match command {
			"connect" => Commands::Connect,
			"quit" => Commands::Quit,
			"join" => Commands::Join,
			"part" => Commands::Part,
			"in" => Commands::In,
			"home" => Commands::Home,
			"send" => Commands::Send,
			"query" => Commands::Query,
			"me" => Commands::Me,
			"nick" => Commands::Nick,
			"away" => Commands::Away,
			"back" => Commands::Back,
			"whois" => Commands::Whois,
			"list" => Commands::List,
			"names" => Commands::Names,
			"topic" => Commands::Topic,
			"motd" => Commands::MotD,
			"ping" => Commands::Ping,
			"raw" => Commands::Raw,
			"help" => Commands::Help,
			_ => Commands::Unknown
		}
	}
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