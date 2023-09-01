use crate::irc::irc_context::IRCContext;
use crate::irc::irc_errors::IRCError;
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
    Unknown,
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
            _ => IRCCommands::Unknown,
        };

        match found {
            IRCCommands::Unknown => Err(IRCError::CommandNotFound(command.to_string())),
            _ => Ok(found),
        }
    }

    fn format(&self, message: &str) -> String {
        match self {
            IRCCommands::Nick => todo!(),
            IRCCommands::Join => todo!(),
            IRCCommands::PrivMsg => todo!(),
            IRCCommands::Notice => todo!(),
            IRCCommands::Ping => todo!(),
            _ => message.to_string(),
        }
    }

    fn craft(
        &self,
        command: &str,
        data: &str,
        ctx: IRCContext,
    ) -> Result<IRCMessageParsed, IRCError> {
        match self {
            IRCCommands::Nick => Ok(craft_nick(data)),
            IRCCommands::User => Ok(craft_user(data, ctx)),
            IRCCommands::Pass => Ok(craft_pass(ctx)),
            IRCCommands::Join => Ok(craft_join(data)),
            IRCCommands::Part => Ok(craft_part(data)),
            IRCCommands::PrivMsg => Ok(craft_privmsg(data, ctx)),
            IRCCommands::Away => Ok(craft_away(data)),
            IRCCommands::Pong => Ok(craft_pong(data)),
            _ => Err(IRCError::CommandNotFound(command.to_string())),
        }
    }
}

fn craft_nick(data: &str) -> IRCMessageParsed {
    IRCMessageParsed {
        prefix: "".to_string(),
        command: "NICK".to_string(),
        target: "".to_string(),
        data: data.to_string(),
    }
}

fn craft_user(data: &str, ctx: IRCContext) -> IRCMessageParsed {
    IRCMessageParsed {
        prefix: "".to_string(),
        command: "USER".to_string(),
        target: ctx.nick.to_string(),
        data: data.to_string(),
    }
}

fn craft_pass(ctx: IRCContext) -> IRCMessageParsed {
    IRCMessageParsed {
        prefix: "".to_string(),
        command: "PASS".to_string(),
        target: "".to_string(),
        data: ctx.password.to_string(),
    }
}

fn add_hash_to_chan(chans: &str) -> String {
    let chans: Vec<String> = chans
        .split(',')
        .map(|chan| {
            if let Some(first_char) = chan.chars().next() {
                match first_char {
                    '&' | '#' | '+' | '!' | '0' => chan.to_string(),
                    _ => format!("#{}", chan),
                }
            } else {
                chan.to_string()
            }
        })
        .collect();
    chans.join(",")
}

fn craft_join(data: &str) -> IRCMessageParsed {
    // TODO consider what to do if keys contain spaces
    let data: Vec<&str> = data.split_whitespace().collect();
    let empty = "";
    let chans: String = add_hash_to_chan(data.get(0).unwrap_or(&empty));
    let keys: &str = data.get(1).unwrap_or(&empty);

    let data = format!("{chans} {keys}");
    IRCMessageParsed {
        prefix: "".to_string(),
        command: "JOIN".to_string(),
        target: "".to_string(),
        data,
    }
}

fn craft_part(data: &str) -> IRCMessageParsed {
    // TODO consider what to do if part message contains :
    let data: Vec<&str> = data.split(":").collect();
    let empty = "";
    let chans: String = add_hash_to_chan(data.get(0).unwrap_or(&empty));
    let message: &str = data.get(1).unwrap_or(&empty);

    let data = format!("{chans} {message}");
    IRCMessageParsed {
        prefix: "".to_string(),
        command: "PART".to_string(),
        target: "".to_string(),
        data,
    }
}

fn craft_privmsg(data: &str, ctx: IRCContext) -> IRCMessageParsed {
    IRCMessageParsed {
        prefix: "".to_string(),
        command: "PRIVMSG".to_string(),
        target: ctx.channel.to_string(),
        data: data.to_string(),
    }
}

fn craft_pong(data: &str) -> IRCMessageParsed {
    IRCMessageParsed {
        prefix: "".to_string(),
        command: "PONG".to_string(),
        target: "".to_string(),
        data: data.to_string(),
    }
}

fn craft_away(data: &str) -> IRCMessageParsed {
    IRCMessageParsed {
        prefix: "".to_string(),
        command: "AWAY".to_string(),
        target: "".to_string(),
        data: data.to_string(),
    }
}
