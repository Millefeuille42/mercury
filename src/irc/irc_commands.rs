use crate::irc::irc_errors::IRCError;
use crate::irc::irc_context::IRCContext;
use crate::irc::irc_message_handler::IRCMessageHandler;
use crate::irc::irc_message_parsed::IRCMessageParsed;

pub enum IRCCommands {
	Pass,
	Nick,
	User,
	Oper,
	Mode,
	Service,
	Quit,
	SQuit,
	Join,
	Part,
	Topic,
	Names,
	List,
	Invite,
	Kick,
	PrivMsg,
	Notice,
	MotD,
	LUsers,
	Version,
	Stats,
	Links,
	Time,
	Connect,
	Trace,
	Admin,
	Info,
	SQuery,
	Who,
	WhoIs,
	WhoWas,
	Kill,
	Ping,
	Pong,
	Error,
	Away,
	Rehash,
	Die,
	Restart,
	Summon,
	Users,
	WallOps,
	UserHost,
	IsOn,
	Unknown
}

impl IRCMessageHandler for IRCCommands {
	fn new(command: &str) -> Result<Self, IRCError> {
		let command = command.to_uppercase();
		let command = command.as_str();
		let found = match command {
			"PASS" => IRCCommands::Pass,
			"NICK" => IRCCommands::Nick,
			"USER" => IRCCommands::User,
			"OPER" => IRCCommands::Oper,
			"MODE" => IRCCommands::Mode,
			"SERVICE" => IRCCommands::Service,
			"QUIT" => IRCCommands::Quit,
			"SQUIT" => IRCCommands::SQuit,
			"JOIN" => IRCCommands::Join,
			"PART" => IRCCommands::Part,
			"TOPIC" => IRCCommands::Topic,
			"NAMES" => IRCCommands::Names,
			"LIST" => IRCCommands::List,
			"INVITE" => IRCCommands::Invite,
			"KICK" => IRCCommands::Kick,
			"PRIVMSG" => IRCCommands::PrivMsg,
			"NOTICE" => IRCCommands::Notice,
			"MOTD" => IRCCommands::MotD,
			"LUSERS" => IRCCommands::LUsers,
			"VERSION" => IRCCommands::Version,
			"STATS" => IRCCommands::Stats,
			"LINKS" => IRCCommands::Links,
			"TIME" => IRCCommands::Time,
			"CONNECT" => IRCCommands::Connect,
			"TRACE" => IRCCommands::Trace,
			"ADMIN" => IRCCommands::Admin,
			"INFO" => IRCCommands::Info,
			"SQUERY" => IRCCommands::SQuery,
			"WHO" => IRCCommands::Who,
			"WHOIS" => IRCCommands::WhoIs,
			"WHOWAS" => IRCCommands::WhoWas,
			"KILL" => IRCCommands::Kill,
			"PING" => IRCCommands::Ping,
			"PONG" => IRCCommands::Pong,
			"ERROR" => IRCCommands::Error,
			"AWAY" => IRCCommands::Away,
			"REHASH" => IRCCommands::Rehash,
			"DIE" => IRCCommands::Die,
			"RESTART" => IRCCommands::Restart,
			"SUMMON" => IRCCommands::Summon,
			"USERS" => IRCCommands::Users,
			"WALLOPS" => IRCCommands::WallOps,
			"USERHOST" => IRCCommands::UserHost,
			"ISON" => IRCCommands::IsOn,
			_ => IRCCommands::Unknown
		};

		match found {
			IRCCommands::Unknown => Err(IRCError::CommandNotFound(command.to_string())),
			_ => Ok(found)
		}
	}

	fn format(&self, message: &str) -> String {
		match self {
			IRCCommands::Nick		=> todo!(),
			IRCCommands::Join		=> todo!(),
			IRCCommands::PrivMsg	=> todo!(),
			IRCCommands::Notice		=> todo!(),
			IRCCommands::Ping		=> todo!(),
			_						=> message.to_string()
		}
	}

	fn craft(&self, command: &str, ctx: &mut IRCContext) -> Result<IRCMessageParsed, IRCError> {
		match self {
			IRCCommands::Nick		=> todo!(),
			IRCCommands::User		=> todo!(),
			IRCCommands::Pass		=> todo!(),
			IRCCommands::Join		=> todo!(),
			IRCCommands::PrivMsg	=> todo!(),
			IRCCommands::Notice		=> todo!(),
			IRCCommands::Pong		=> todo!(),
			_ 						=> Err(IRCError::CommandNotFound(command.to_string()))
		}
	}
}