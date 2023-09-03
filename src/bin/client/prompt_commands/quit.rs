use std::error::Error;
use clap::{ArgMatches, Command};
use crate::utils::irc_comm_channels::IRCCommChannels;

pub async fn quit<'a>(args: Vec<&str>, channels: IRCCommChannels<'a>) -> Result<(), Box<dyn Error>> {
	let _: ArgMatches = Command::new("join")
		.try_get_matches_from_mut(args)?;

	channels.disconnect().await?;
	Ok(())
}
