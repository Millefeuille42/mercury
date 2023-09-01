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