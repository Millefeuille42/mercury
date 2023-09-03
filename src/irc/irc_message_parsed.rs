use regex::{Match, Regex};
use crate::irc::irc_errors::IRCError;
use crate::irc::irc_context::IRCContext;
use crate::irc::irc_commands::IRCCommands;
use crate::irc::irc_message_handler::IRCMessageHandler;
use crate::irc::irc_replies::IRCReplies;

fn cap_to_string(cap: Option<Match>) -> String {
	match cap {
		None => "".to_string(),
		Some(cap) => cap.as_str().to_string()
	}
}

//
pub struct IRCPrefixParsed {
	pub(crate) nick: String,
	pub(crate) user: String,
	pub(crate) host: String,
}

impl IRCPrefixParsed {
	pub fn clone(&self) -> Self {
		IRCPrefixParsed {
			nick: self.nick.clone(),
			user: self.user.clone(),
			host: self.host.clone(),
		}
	}

	pub fn parse(prefix: String) -> Self {
		let re = Regex::new(
			r"(?P<nick>[^!@\s]+)(!?(?P<user>[^@\s]+))?(@(?P<host>\S+))?"
		).unwrap();
		let Some(caps) = re.captures(prefix.as_str()) else {
			return IRCPrefixParsed {
				nick: "".to_string(),
				user: "".to_string(),
				host: "".to_string(),
			}
		};

		IRCPrefixParsed {
			nick: cap_to_string(caps.name("nick")),
			user: cap_to_string(caps.name("user")),
			host: cap_to_string(caps.name("host")),
		}
	}
}

pub struct IRCMessageParsed {
	pub prefix: String,
	pub command: String,
	pub target: String,
	pub data: String
}

impl IRCMessageParsed {
	pub fn clone(&self) -> Self {
		IRCMessageParsed {
			prefix: self.prefix.clone(),
			command: self.command.clone(),
			target: self.target.clone(),
			data: self.data.clone(),
		}
	}

	pub fn parse_prefix(&self) -> IRCPrefixParsed {
		IRCPrefixParsed::parse(self.prefix.clone())
	}

	pub fn parse(raw_message: String) -> Self {
		let re = Regex::new(
			r"(:(?P<prefix>\S+)\s)?((?P<command>\S+)\s)?((?P<target>[^:\s]+)\s)?:?(?P<data>.+)?"
		).unwrap();
		let Some(caps) = re.captures(raw_message.as_str()) else {
			println!("[DEBUG] got no match with regex");
			return IRCMessageParsed {
				prefix: "".to_string(),
				command: "".to_string(),
				target: "".to_string(),
				data: "".to_string(),
			}
		};

		//println!("[DEBUG] got matches with regex");
		//println!("[DEBUG] command is <{}>", cap_to_string(caps.name("command")));
		IRCMessageParsed {
			prefix: cap_to_string(caps.name("prefix")),
			command: cap_to_string(caps.name("command")),
			target: cap_to_string(caps.name("target")),
			data: cap_to_string(caps.name("data")),
		}
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
			Ok(val) => val.format(self.clone()),
			Err(_) => match IRCReplies::new(command) {
				Ok(val) => val.format(self.clone()),
				Err(_) => IRCCommands::Unknown.format(self.clone())
			},
		}
	}
}
