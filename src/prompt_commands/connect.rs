use clap::{Command, arg, ArgAction, ArgMatches};
use std::error::Error;
use std::string::String;
use crate::utils::irc_comm_channels::IRCCommChannels;

pub async fn connect<'a>(args: Vec<&str>, channels: IRCCommChannels<'a>) -> Result<(), Box<dyn Error>> {
	let matches: ArgMatches = Command::new("connect")
		.args(&[
			arg!(-H --host <HOSTNAME_OR_IP> "irc.libera.chat"),
			arg!(-p --port [PORT] "6667")
				.default_value_if("tls", "false", Some("6667"))
				.default_value_if("tls", "true", Some("6697"))
				.default_value(Some("6667")),
			arg!(-P --password [PASSWORD] "MyPassword"),
			arg!(-n --nick [NICK] "mercury")
				.default_value(Some("mercury")),
			arg!(--tls)
				.action(ArgAction::SetTrue),
		])
		.try_get_matches_from_mut(args)?;

	let host: &String = matches.try_get_one::<String>("host")?
		.ok_or("missing host!".to_string())?;
	let port: &String = matches.try_get_one::<String>("port")?
		.ok_or("missing port!".to_string())?;
	let nick: &String = matches.try_get_one::<String>("nick")?
		.ok_or("missing nick!".to_string())?;
	let password: Option<&String> = matches.try_get_one::<String>("password")?;

	let empty = "".to_string();
	println!("{nick}:{}@{host}:{port}", password.unwrap_or(&empty));


	channels.connect(format!("{host}:{port}")).await?;
	channels.write(format!("NICK {nick}").as_str()).await?;
	channels.write(format!("USER {nick} 0 * :{nick}").as_str()).await?;

	Ok(())
}
