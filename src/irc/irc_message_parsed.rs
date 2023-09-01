use crate::irc::irc_errors::IRCError;
use crate::irc::irc_context::IRCContext;
use crate::irc::irc_commands::IRCCommands;
use crate::irc::irc_message_handler::IRCMessageHandler;
use crate::irc::irc_replies::IRCReplies;

// TODO might set as option since none of the fields are mandatory
pub struct IRCMessageParsed {
	pub(crate) prefix: String,
	pub(crate) command: String,
	pub(crate) target: String,
	pub(crate) data: String
}

impl IRCMessageParsed {
	pub fn parse(_: String) -> Self {
		// TODO parsing with regex, isolating in the proper fields with named capture groups
		todo!()
	}

	pub fn as_raw(&self) -> String {
		format!(
			"{}{}{}{}{}{}{}",
			self.prefix,
			if !self.prefix.is_empty() { " " } else { "" },
			self.command,
			if !self.command.is_empty() { " " } else { "" },
			self.target,
			if !self.target.is_empty() { " " } else { "" },
			self.data
		)
	}

	pub fn craft(command: &str, data: &str, ctx: IRCContext) -> Result<IRCMessageParsed, IRCError> {
		match IRCCommands::new(command) {
			Ok(val) => val.craft(command, data, ctx),
			Err(_) => match IRCReplies::new(command) {
				Ok(val) => val.craft(command, data, ctx),
				Err(_) => IRCCommands::Unknown.craft(command, data, ctx)
			},
		}
	}

	pub fn as_formatted(&self) -> String {
		let command = self.command.as_str();
		match IRCCommands::new(command) {
			Ok(val) => val.format(command),
			Err(_) => match IRCReplies::new(command) {
				Ok(val) => val.format(command),
				Err(_) => IRCCommands::Unknown.format(command)
			},
		}
	}
}
