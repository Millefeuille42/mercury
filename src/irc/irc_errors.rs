use std::fmt::{Debug, Display, Formatter};
#[derive(Debug)]
pub enum IRCError {
	CommandNotFound(String),
	ReplyNotFound(String),
	NoMessageContent
}

impl Display for IRCError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			IRCError::CommandNotFound(message) => write!(f, "command not found: {}", message),
			IRCError::ReplyNotFound(message) => write!(f, "reply not found: {}", message),
			IRCError::NoMessageContent => write!(f, "no message provided"),
		}
	}
}

impl std::error::Error for IRCError {}
