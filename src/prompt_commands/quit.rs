use std::error::Error;
use crate::utils::irc_comm_channels::IRCCommChannels;

pub async fn quit<'a>(_: Vec<&str>, channels: IRCCommChannels<'a>) -> Result<(), Box<dyn Error>> {
	channels.disconnect().await?;
	Ok(())
}
