use std::error::Error;
use clap::{arg, ArgMatches, Command};
use crate::irc::irc_context::IRCContext;
use crate::utils::irc_comm_channels::IRCCommChannels;
use crate::irc::irc_message_parsed::IRCMessageParsed;
use crate::prompt_commands::utils::matches_to_string;

pub async fn query<'a>(
	args: Vec<&str>,
	channels: IRCCommChannels<'a>,
	ctx: &mut IRCContext
) -> Result<(), Box<dyn Error>> {
	let matches: ArgMatches = Command::new("query")
		.args(&[
			arg!(<user> "user"),
			arg!(<message> ... "message to send").trailing_var_arg(true)
		])
		.try_get_matches_from_mut(args)?;

	let chan = matches
		.try_get_one::<String>("user")?
		.ok_or("missing user!".to_string())?;

	// TODO Match against user regex
	ctx.channel = chan.to_string();
	if let Some(message) = matches.try_get_many::<String>("message")? {
		let message = matches_to_string(message.collect::<Vec<&String>>());
		let irc_message = IRCMessageParsed {
			prefix: "".to_string(),
			command: "PRIVMSG".to_string(),
			target: chan.to_string(),
			data: format!(":{message}").to_string(),
		};
		channels.write(irc_message.as_raw().as_str()).await?;
	}

	Ok(())
}
