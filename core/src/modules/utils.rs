use futures_util::future;
use futures_util::stream::{SplitStream, Stream, StreamExt};
use uuid::Uuid;
use warp::filters::ws::{Message, WebSocket};
use warp::Error;

use super::input::{ClientInput, Input, InputErrors};

pub fn read(id: Uuid, ws_stream: SplitStream<WebSocket>) -> impl Stream<Item = Result<ClientInput, Error>> {
    ws_stream
        .filter(|x| {
            future::ready(match x.as_ref() {
                Ok(msg) => msg.is_text(),
                Err(_) => false,
            })
        })
        .map(move |x: Result<Message, Error>| match x {
            Ok(msg) => {
                let input: Input = serde_json::from_str(msg.to_str().unwrap_or("error")).unwrap_or_else(|err| {
                    println!("ERR: Error parsing input from client {id} with error: {err:#?}");
                    Input::Error(InputErrors::InputParseError)
                });

                Ok(ClientInput::new(id, input))
            }
            Err(err) => Err(err),
        })
}

pub fn log_warp_message(id: Uuid, msg: &Message) {
    if let Ok(raw) = msg.to_str() {
        println!("INFO: Sending JSON(\n{raw:#?}\n) to client {id}");
    } else {
        println!("ERR: Error parsing warp message {msg:#?}");
    }
}
