use std::error::Error;
use crate::utils::irc_comm_channels::IRCCommChannels;

pub async fn quit<'a>(_: Vec<&str>, _: IRCCommChannels<'a>) -> Result<(), Box<dyn Error>> {
	todo!()
}
