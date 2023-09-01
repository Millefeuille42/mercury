use crate::utils::irc_comm_channels::IRCCommChannels;
use crate::irc::irc_message_parsed::IRCMessageParsed;
use crate::irc::irc_context::IRCContext;
use clap::{arg, ArgMatches, Command};
use std::error::Error;
use std::string::String;

pub async fn join<'a>(args: Vec<&str>, channels: IRCCommChannels<'a>, ctx: &mut IRCContext) -> Result<(), Box<dyn Error>> {
	let matches: ArgMatches = Command::new("join")
		.args(&[
			arg!(<channel> ... "#channel:key #channel2:key2"),
		])
		.try_get_matches_from_mut(args)?;

	let chans: Vec<String> = matches.try_get_many("channel")?
		.ok_or("no channel provided!".to_string())?
		.collect::<Vec<&String>>()
		.iter()
		.map(|c| c.to_string())
		.collect();

	// TODO Send one message for all
	for chan in chans {
		let ctx_: IRCContext = ctx.clone();
		let chan_key: Vec<&str> = chan.split(":").collect();
		let chan: &str = chan_key.get(0).ok_or("chan is invalid")?;
		ctx.channel = chan.to_string();
		if let Some(key) = chan_key.get(1) {
			ctx.channel = format!("{} {}", chan, key);
		}
		let message = IRCMessageParsed::craft("JOIN", ctx.channel.as_str(), ctx_)?;
		channels.write(message.as_raw().as_str()).await?;
	}

    Ok(())
}
