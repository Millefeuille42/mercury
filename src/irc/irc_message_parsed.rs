use crate::irc::irc_errors::IRCError;
use crate::irc::irc_context::IRCContext;
use crate::irc::irc_commands::IRCCommands;
use crate::irc::irc_message_handler::IRCMessageHandler;
use crate::irc::irc_replies::IRCReplies;

// TODO might set as option since none of the fields are mandatory
pub struct IRCMessageParsed {
	prefix: String,
	command: String,
	target: String,
	data: String
}

impl IRCMessageParsed {
	pub fn parse(raw: String) -> Self {
		// TODO parsing with regex, isolating in the proper fields with named capture groups
		todo!()
	}

	pub fn as_raw(&self) -> String {
		format!("{} {} {} {}", self.prefix, self.command, self.target, self.data)
	}

	pub fn craft(command: &str, ctx: &mut IRCContext) -> Result<IRCMessageParsed, IRCError> {
		match IRCCommands::new(command) {
			Ok(val) => val.craft(command, ctx),
			Err(_) => match IRCReplies::new(command) {
				Ok(val) => val.craft(command, ctx),
				Err(_) => IRCCommands::Unknown.craft(command, ctx)
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
