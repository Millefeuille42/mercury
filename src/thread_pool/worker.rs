use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::Builder;
use crate::thread_pool::job::Job;

pub(crate) struct Worker {
	pub(crate) id: usize,
	pub(crate) thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	pub(crate) fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Result<Worker, std::io::Error> {
		let builder = Builder::new();
		let thread = builder.spawn(move || loop {
			let message = receiver.lock().unwrap().recv();

			match message {
				Ok(job) => {
					println!("Worker {id} got a job; executing.");

					job();
				}
				Err(_) => {
					println!("Worker {id} disconnected; shutting down.");
					break;
				}
			}
		})?;

		Ok(Worker { id, thread: Some(thread) })
	}
}
