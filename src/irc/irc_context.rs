// TODO add connected status
pub struct IRCContext {
	pub server: String,
	pub nick: String,
	pub password: String,
	pub channel: String,
}

impl IRCContext {
	pub fn new() -> Self {
		IRCContext {
			server: "".to_string(),
			nick: "".to_string(),
			password: "".to_string(),
			channel: "".to_string(),
		}
	}

	pub fn clone(&mut self) -> Self {
		IRCContext {
			server: self.server.clone(),
			nick: self.nick.clone(),
			password: self.password.clone(),
			channel: self.channel.clone(),
		}
	}
}