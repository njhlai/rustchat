use std::sync::Arc;
use std::sync::mpsc::channel;

use super::hub::Hub;
use super::input::Input;

pub struct Server {
	port: u16,
	hub: Arc<Hub>,
}

impl Server {
	pub fn new(port: u16) -> Self {
		Server {
			port: port,
			hub: Arc::new(Hub::new()),
		}
	}

	pub fn run(&self) {
		let (tx, rx) = channel::<Input>();
	}
}