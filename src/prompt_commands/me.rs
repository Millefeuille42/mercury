use std::error::Error;
use clap::{arg, ArgMatches, Command};
use crate::irc::irc_context::IRCContext;
use crate::utils::irc_comm_channels::IRCCommChannels;
use crate::irc::irc_errors::IRCError::NoMessageContent;
use crate::irc::irc_message_parsed::IRCMessageParsed;

pub async fn me<'a>(
	args: Vec<&str>,
	channels: IRCCommChannels<'a>,
	ctx: IRCContext
) -> Result<(), Box<dyn Error>> {
	let args_ = args.clone();
	let _: ArgMatches = Command::new("me")
		.args(&[
			// TODO finish that (returns a Vec, join it and it's done)
			arg!(<message> ... "message to send").trailing_var_arg(true)
		])
		.try_get_matches_from_mut(args)?;

	if args_.len() <= 2 {
		return Err(Box::try_from(NoMessageContent).unwrap())
	}

	let message = args_[2..].join(" ");
	let irc_message = IRCMessageParsed {
		prefix: "".to_string(),
		command: "PRIVMSG".to_string(),
		target: ctx.channel,
		data: format!(":\x01ACTION {message}\x01").to_string(),
	};
	channels.write(irc_message.as_raw().as_str()).await?;

	Ok(())
}
