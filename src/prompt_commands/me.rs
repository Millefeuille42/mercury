use std::error::Error;
use clap::{arg, ArgMatches, Command};
use crate::irc::irc_context::IRCContext;
use crate::utils::irc_comm_channels::IRCCommChannels;
use crate::irc::irc_errors::IRCError::NoMessageContent;
use crate::irc::irc_message_parsed::IRCMessageParsed;
use crate::prompt_commands::utils::matches_to_string;

pub async fn me<'a>(
	args: Vec<&str>,
	channels: IRCCommChannels<'a>,
	ctx: IRCContext
) -> Result<(), Box<dyn Error>> {
	let matches: ArgMatches = Command::new("me")
		.args(&[
			arg!(<message> ... "message to send").trailing_var_arg(true)
		])
		.try_get_matches_from_mut(args)?;

	let message = matches_to_string(matches.try_get_many("message")?
		.ok_or(NoMessageContent)?
		.collect::<Vec<&String>>());
	let irc_message = IRCMessageParsed {
		prefix: "".to_string(),
		command: "PRIVMSG".to_string(),
		target: ctx.channel,
		data: format!(":\x01ACTION {message}\x01").to_string(),
	};
	channels.write(irc_message.as_raw().as_str()).await?;

	Ok(())
}
