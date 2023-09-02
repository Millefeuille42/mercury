use std::error::Error;
use clap::{arg, ArgMatches, Command};
use crate::utils::irc_comm_channels::IRCCommChannels;
use crate::irc::irc_errors::IRCError::NoMessageContent;
use crate::irc::irc_message_parsed::IRCMessageParsed;
use crate::prompt_commands::utils::matches_to_string;

pub async fn send<'a>(args: Vec<&str>, channels: IRCCommChannels<'a>) -> Result<(), Box<dyn Error>> {
	let matches: ArgMatches = Command::new("send")
		.args(&[
			arg!(<channel> "#channel / user"),
			arg!(<message> ... "message to send").trailing_var_arg(true)
		])
		.try_get_matches_from_mut(args)?;

	// TODO match against channel regex
	let chan = matches
		.try_get_one::<String>("channel")?
		.ok_or("missing channel!".to_string())?;

	let message = matches_to_string(matches.try_get_many("message")?
		.ok_or(NoMessageContent)?
		.collect::<Vec<&String>>());
	let irc_message = IRCMessageParsed {
		prefix: "".to_string(),
		command: "PRIVMSG".to_string(),
		target: chan.to_string(),
		data: format!(":{message}").to_string(),
	};
	channels.write(irc_message.as_raw().as_str()).await?;

	Ok(())
}
