use crate::irc::irc_errors::IRCError;
use crate::irc::irc_context::IRCContext;
use crate::irc::irc_message_parsed::IRCMessageParsed;

pub trait IRCMessageHandler {
	fn new(command: &str) -> Result<Self, IRCError> where Self: Sized;
	fn format(&self, message: IRCMessageParsed) -> String;
	fn craft(&self, command: &str, data: &str, ctx: IRCContext) -> Result<IRCMessageParsed, IRCError>;
}