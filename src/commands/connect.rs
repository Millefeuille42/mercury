use std::error::Error;
use clap::{Command, arg, ArgAction, ArgMatches};

pub fn connect(args: Vec<&str>) -> Result<(), Box<dyn Error>> {
	println!("In connect");

	let matches: ArgMatches = Command::new("connect")
		.args(&[
			arg!(-H --host <HOSTNAME_OR_IP> "irc.libera.chat"),
			arg!(-p --port [PORT] "6667")
				.default_value_if("tls", "false", Some("6667"))
				.default_value_if("tls", "true", Some("6697")),
			arg!(-P --password [PASSWORD]  "MyPassword"),
			arg!(-n --nick [NICK] "millefy"),
			arg!(--tls)
				.action(ArgAction::SetTrue),
		])
		.get_matches_from(args);

	println!("Got matches");
	let host: &String = matches.try_get_one::<String>("host")?
		.ok_or(format!("missing host!"))?;

	println!("{}", host);

	println!("Done");
	Ok(())
}
