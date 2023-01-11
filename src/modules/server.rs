use std::sync::Arc;
use std::time::Duration;

use futures::future::ready;
use futures::sink::SinkExt;
use futures::stream::{Stream, StreamExt, SplitStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use uuid::Uuid;
use warp::{Error, Filter};
use warp::filters::ws::{Message, WebSocket, Ws};

use super::hub::Hub;
use super::input::{ClientInput, Input, InputErrors};

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

        let (_, serving) = warp::serve(feed).bind_with_graceful_shutdown(
            ([127, 0, 0, 1], self.port),
            async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to install CTRL+C signal handler");
            }
        );

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
                .for_each(move |input: Result<ClientInput, warp::Error>| {
                    client_input_sender.send(input.unwrap()).unwrap();
                    ready(())
                })
        );

        loop {
            let msgs: Vec<Message> = rx.try_iter()
                .map(|msg| Message::text(serde_json::to_string(&msg).unwrap()))
                .collect();

            for msg in msgs {
                ws_sink.send(msg).await.unwrap();
            }

            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }

    fn read(id: Uuid, ws_stream: SplitStream<WebSocket>) -> impl Stream<Item = Result<ClientInput, Error>> {
        ws_stream
            .filter(|x| ready(x.as_ref().unwrap().is_text()))
            .map(move |x: Result<Message, Error>| match x {
                Ok(msg) => {
                    let input: Input = serde_json::from_str(msg.to_str().unwrap_or("error"))
                        .unwrap_or(Input::Error(InputErrors::InputParseError));

                    Ok(ClientInput { id, input })
                },
                Err(err) => Err(err),
            })
    }
}