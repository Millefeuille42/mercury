use crate::utils::irc_comm_channels::IRCCommChannels;
use clap::{arg, Arg, ArgAction, ArgMatches, Command};
use std::error::Error;
use std::string::String;
use crate::irc::irc_context::IRCContext;
use crate::irc::irc_message_parsed::IRCMessageParsed;

pub async fn connect<'a>(
    args: Vec<&str>,
    channels: IRCCommChannels<'a>,
    ctx: &mut IRCContext
) -> Result<(), Box<dyn Error>> {
    let matches: ArgMatches = Command::new("connect")
        .args(&[
            // TODO <host>
			Arg::new("host")
				.required(true),
            arg!(-p --port [PORT] "6667")
                .default_value_if("tls", "false", Some("6667"))
                .default_value_if("tls", "true", Some("6697"))
                .default_value(Some("6667")),
            // TODO This is the detail, not an example
            arg!(-P --password [PASSWORD] "MyPassword"),
            arg!(-n --nick [NICK] "mercury")
                .default_value(Some("mercury")),
            arg!(--tls).action(ArgAction::SetTrue),
        ])
        .try_get_matches_from_mut(args)?;

    let host: &String = matches
        .try_get_one::<String>("host")?
        .ok_or("missing host!".to_string())?;
    let port: &String = matches
        .try_get_one::<String>("port")?
        .ok_or("missing port!".to_string())?;
    let nick: &String = matches
        .try_get_one::<String>("nick")?
        .ok_or("missing nick!".to_string())?;
    let password: Option<&String> = matches.try_get_one::<String>("password")?;

    let empty = "".to_string();
    println!("{nick}:{}@{host}:{port}", password.unwrap_or(&empty));

    channels.connect(format!("{host}:{port}")).await?;

    let message = IRCMessageParsed::craft(
        "NICK", nick, ctx.clone()
    )?;
    channels.write(message.as_raw().as_str()).await?;
    ctx.nick = nick.to_string();

    let message = IRCMessageParsed::craft(
        "USER", format!("{} 0 * :{}", nick, nick).as_str(), ctx.clone()
    )?;
    channels.write(message.as_raw().as_str()).await?;

    Ok(())
}
