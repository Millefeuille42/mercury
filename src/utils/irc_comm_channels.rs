use tokio::sync::{mpsc};
use tokio::sync::mpsc::error::SendError;

pub struct IRCCommChannels<'a> {
	send_tx: &'a mpsc::Sender<String>,
	conn_tx: &'a mpsc::Sender<String>
}

pub fn spawn_channel() -> (mpsc::Sender<String>, mpsc::Receiver<String>) {
	mpsc::channel::<String>(4)
}

impl<'a> IRCCommChannels<'a> {
	pub fn new(send_tx: &'a mpsc::Sender<String>, conn_tx: &'a mpsc::Sender<String>) -> Self {
		IRCCommChannels {
			send_tx,
			conn_tx,
		}
	}

	pub async fn connect(&self, addr: String) -> Result<(), SendError<String>> {
		//println!("[DEBUG] Sending connect signal");
		self.conn_tx.send(addr).await
	}

	pub async fn disconnect(&self) -> Result<(), SendError<String>> {
		self.conn_tx.send("disconnect".to_string()).await
	}

	pub async fn write(&self, buf: &str) -> Result<(), SendError<String>> {
		//println!("[DEBUG] Sending message signal");
		self.send_tx.send(format!("{}\r\n", buf)).await
	}
}
