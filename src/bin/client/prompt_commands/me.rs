use std::error::Error;
use clap::{arg, ArgMatches, Command};
use mercury::irc;
use mercury::irc::Error as IRCError;
use crate::utils::irc_comm_channels::IRCCommChannels;
use crate::prompt_commands::utils::matches_to_string;

pub async fn me<'a>(
	args: Vec<&str>,
	channels: IRCCommChannels<'a>,
	ctx: irc::Context
) -> Result<(), Box<dyn Error>> {
	let matches: ArgMatches = Command::new("me")
		.args(&[
			arg!(<message> ... "message to send").trailing_var_arg(true)
		])
		.try_get_matches_from_mut(args)?;

	let message = matches_to_string(matches.try_get_many("message")?
		.ok_or(IRCError::NoMessageContent)?
		.collect::<Vec<&String>>());
	let irc_message = irc::Message {
		prefix: "".to_string(),
		command: "PRIVMSG".to_string(),
		target: ctx.channel,
		data: format!(":\x01ACTION {message}\x01").to_string(),
	};
	channels.write(irc_message.as_raw().as_str()).await?;

	Ok(())
}
