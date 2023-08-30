mod commands;

use std::collections::HashMap;
use std::error::Error;
use std::io;

const CONFIG_FILE: &str = "mercury/conf.ini";

struct UserConfigDir;

impl UserConfigDir {
	fn get() -> Option<String> {
		if cfg!(target_os = "windows") {
			std::env::var("APPDATA").ok()
		} else if cfg!(target_os = "linux") {
			std::env::var("HOME").map(|home| format!("{}/.config/", home)).ok()
		} else {
			None
		}
	}

	fn get_config_file() -> Option<String> {
		match UserConfigDir::get() {
			Some(config_dir) => Some(format!("{}{}", config_dir, CONFIG_FILE)),
			None => None
		}
	}
}

fn command_manager(command: &str) {
	static mut COMMANDS: Option<HashMap<&'static str, fn(Vec<&str>) -> Result<(), Box<dyn Error>>>> = None;

	let mut command = command.trim();
	if ! command.starts_with("/") {
		return;
	}
	command = command.trim_matches('/');

	let args: Vec<&str> = command.split_whitespace().collect();

	unsafe {
		if COMMANDS.is_none() {
			println!("Init command");
			let mut map = HashMap::new();
			map.insert("connect", commands::connect::connect as fn(Vec<&str>) -> Result<(), Box<dyn Error>>);
			map.insert("quit", commands::quit::quit);
			COMMANDS = Some(map);
		}

		if let Some(command_map) = &COMMANDS {
			if let Some(command_func) = command_map.get(args[0]) {
				println!("Found command");
				match command_func(args) {
					Err(err) => {eprintln!("{}", err.to_string())}
					_ => {}
				}
			} else {
				println!("Invalid command");
			}
		}
	}
}

fn main() {
	loop {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");

		let input = input.trim();
		if input.eq_ignore_ascii_case("exit") {
			break;
		}

		command_manager(input);
	}
}