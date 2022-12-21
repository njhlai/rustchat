use std::sync::Arc;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

use uuid::Uuid;
use warp::Filter;
use warp::filters::ws::WebSocket;
use warp::ws::Ws;

use super::hub::Hub;
use super::input::Input;

pub struct ClientInput {
	pub id: Uuid,
	pub input: Input,
}

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
		let (tx, rx) = sync_channel::<ClientInput>(32);

		let feed = warp::path("feed")
			.and(warp::ws())
			.and(warp::any().map(move || tx.clone()))
			.and(warp::any().map(move || self.hub.clone()))
			.map(
				move |ws: Ws, client_input_sender, hub| {
					ws.on_upgrade(move |web_socket| async move {
						thread::spawn(move || {
							Self::process_client(web_socket, client_input_sender, hub);
						});
					})
				}
			);

		self.hub.run(rx);
	}

	fn process_client(web_socket: WebSocket, client_input_sender: SyncSender<ClientInput>, hub: Arc<Hub>) {
	}
}