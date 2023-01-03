use std::sync::Arc;
use std::{time::Duration};

use futures::sink::SinkExt;
use futures::stream::{Stream, StreamExt, SplitStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use uuid::Uuid;
use warp::Filter;
use warp::filters::ws::{Message, WebSocket};
use warp::ws::Ws;

use super::hub::Hub;
use super::input::{Input, InputErrors};

#[derive(Debug)]
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

	pub async fn run(&self) {
		let (tx, rx) = unbounded_channel::<ClientInput>();

		let hub = self.hub.clone();

		let feed = warp::path("feed")
			.and(warp::ws())
			.and(warp::any().map(move || tx.clone()))
			.and(warp::any().map(move || hub.clone()))
			.map(
				move |ws: Ws, client_input_sender, hub| {
					ws.on_upgrade(move |web_socket| async {
						tokio::spawn(
							Self::process_client(web_socket, client_input_sender, hub)
						);
					})
				}
			);

		let ( _, serving) = warp::serve(feed).bind_with_graceful_shutdown(([127, 0, 0, 1], self.port), async {
			tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
		});

		tokio::select! {
			_ = serving => {},
			_ = self.hub.run(rx) => {}
		}
	}

	async fn process_client(web_socket: WebSocket, client_input_sender: UnboundedSender<ClientInput>, hub: Arc<Hub>) {
		let (mut ws_sink, ws_stream) = web_socket.split();
		let id = Uuid::new_v4();
		let rx = hub.connect(id);

		tokio::spawn(
			Self::read(id, ws_stream)
				.for_each(move |input| {
					client_input_sender.send(input.unwrap());
					futures::future::ready(())
				})
		);

		loop {
			for msg in rx.try_iter() {
				let json_output = serde_json::to_string(&msg);
				println!("Read json: {:#?}", json_output);
				ws_sink.send(Message::text(json_output.unwrap()));
			}

			tokio::time::sleep(Duration::from_millis(200)).await;
		}
	}

	fn read(id: Uuid, ws_stream: SplitStream<WebSocket>) -> impl Stream<Item = Result<ClientInput, Error>> {
		ws_stream
			.filter(|x| futures::future::ready(x.as_ref().unwrap().is_text()))
			.map(move |x: Result<Message, Error>| match x {
				Ok(msg) => {
					let input: Input = serde_json::from_str(msg.to_str().unwrap_or("error"))
						.unwrap_or(Input::Error(InputErrors::InputParseError));

					Ok(ClientInput {
						id: id,
						input: input,
					})
				},
				Err(err) => Err(err),
			})
	}
}