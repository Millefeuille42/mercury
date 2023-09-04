mod worker;
mod job;

use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use job::Job;
use worker::Worker;

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: Option<Sender<Job>>,
}

impl ThreadPool {
	pub fn new(size: usize) -> Result<ThreadPool, std::io::Error> {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));
		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			let worker = Worker::new(id, Arc::clone(&receiver))?;
			workers.push(worker);
		}

		Ok(ThreadPool { workers, sender: Some(sender) })
	}

	pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
		let job = Box::new(f);

		self.sender.as_ref().unwrap().send(job).unwrap();
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		drop(self.sender.take());
		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}