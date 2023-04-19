use std::sync::Arc;
use std::time::Duration;

use futures_util::future;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use uuid::Uuid;
use warp::filters::ws::{Message, WebSocket, Ws};
use warp::Filter;

use super::hub::Hub;
use super::input::ClientInput;
use super::utils;

pub struct Server {
    port: u16,
    hub: Arc<Hub>,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port, hub: Arc::new(Hub::new()) }
    }

    pub async fn run(&self) {
        let (tx, rx) = unbounded_channel::<ClientInput>();
        let hub = self.hub.clone();

        let feed = warp::path("feed")
            .and(warp::ws())
            .and(warp::any().map(move || tx.clone()))
            .and(warp::any().map(move || hub.clone()))
            .map(move |ws: Ws, client_input_sender, hub| {
                ws.on_upgrade(move |web_socket| async {
                    tokio::spawn(Self::process_client(web_socket, client_input_sender, hub));
                })
            });

        let (_, serving) = warp::serve(feed).bind_with_graceful_shutdown(([127, 0, 0, 1], self.port), async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        });

        tokio::select! {
            _ = serving => { println!(); },
            _ = self.hub.run(rx) => {}
        }
    }

    async fn process_client(web_socket: WebSocket, client_input_sender: UnboundedSender<ClientInput>, hub: Arc<Hub>) {
        let (mut ws_sink, ws_stream) = web_socket.split();
        let id = Uuid::new_v4();
        let rx = hub.connect(id);

        tokio::spawn(utils::read(id, ws_stream).for_each(move |input: Result<ClientInput, warp::Error>| {
            if let Ok(msg) = input {
                println!("INFO: Received message {:#?} from client {id}", &msg);
                client_input_sender.send(msg).unwrap_or_else(|err| {
                    println!("ERR: Error receiving message from client {id} with error: {err:#?}");
                });
            }
            future::ready(())
        }));

        loop {
            let msgs: Vec<Message> = rx
                .try_iter()
                .map(|msg| {
                    println!("INFO: Sending message {msg:#?} to client {id}");
                    Message::text(serde_json::to_string(&msg).unwrap_or_else(|_| "Output Parse Error".to_string()))
                })
                .collect();

            for msg in msgs {
                utils::log_warp_message(id, &msg);
                ws_sink.send(msg).await.unwrap_or_else(|err| {
                    println!("ERR: Error sending message to client {id} with error: {err:#?}");
                });
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }
}
