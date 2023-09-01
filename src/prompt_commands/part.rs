use crate::utils::irc_comm_channels::IRCCommChannels;
use crate::irc::irc_message_parsed::IRCMessageParsed;
use crate::irc::irc_context::IRCContext;
use clap::{arg, ArgMatches, Command};
use std::error::Error;
use std::string::String;

pub async fn part<'a>(args: Vec<&str>, channels: IRCCommChannels<'a>, ctx: &mut IRCContext) -> Result<(), Box<dyn Error>> {
	let matches: ArgMatches = Command::new("part")
		.args(&[
			arg!([channel] ... "#channel #channel2"),
			// TODO set message as last / raw (after --)
			arg!(-m --message [message] "leaving, bye !"),
		])
		.try_get_matches_from_mut(args)?;

	let chans = matches.try_get_many("channel")?;
	let chan_list = if chans.is_some() {
		let chans: Vec<String> = chans.ok_or("no channel provided!".to_string())?
			.collect::<Vec<&String>>()
			.iter()
			.map(|c| c.to_string())
			.collect();
		if chans.contains(&ctx.channel) {
			ctx.channel = "".to_string();
		}
		chans.join(",")
	} else {
		let ret = ctx.channel.clone();
		ctx.channel = "".to_string();
		ret
	};

	let final_message = if let Some(part_message) = matches.try_get_one::<String>("message")? {
		format!("{} :{}", chan_list, part_message)
	} else {
		chan_list
	};

	let ctx_: IRCContext = ctx.clone();
	let message = IRCMessageParsed::craft("JOIN", final_message.as_str(), ctx_)?;
	channels.write(message.as_raw().as_str()).await?;

    Ok(())
}
