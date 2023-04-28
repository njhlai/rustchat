use std::collections::{hash_map::Entry, HashMap};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::RwLock;

use futures_util::stream::StreamExt;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;

use super::data::{Feed, Message, User};
use super::input::{ClientInput, Input, Join, Post};
use super::output::{CurrentState, Output, OutputErrors, Posted, UserActivityTimestamp};

pub struct Hub {
    feed: RwLock<Feed>,
    users: RwLock<HashMap<Uuid, User>>,
    outpost: RwLock<HashMap<Uuid, SyncSender<Output>>>,
}

impl Hub {
    pub fn new() -> Self {
        Hub { feed: RwLock::default(), users: RwLock::default(), outpost: RwLock::default() }
    }

    pub async fn run(&self, rx: UnboundedReceiver<ClientInput>) {
        UnboundedReceiverStream::new(rx).for_each(|input| self.process(input)).await;

        println!("INFO: Hub shutting down");
    }

    pub fn send(&self, output: &Output) {
        self.outpost.read().unwrap().values().for_each(|output_sender| {
            output_sender.send(output.clone()).unwrap_or_else(|err| {
                println!("ERR: Internal hub error sending message to all clients with error: {err:#?}");
            });
        });
    }

    pub fn send_to_user(&self, id: Uuid, output: Output) {
        match self.outpost.read().unwrap().get(&id) {
            Some(x) => x.send(output).unwrap_or_else(|err| {
                println!("ERR: Internal hub error sending message to client {id} with error: {err:#?}");
            }),
            None => {
                println!("ERR: Internal hub error, can't find client {id} to send message");
            }
        }
    }

    pub fn send_to_complement(&self, id: Uuid, output: &Output) {
        self.outpost
            .read()
            .unwrap()
            .iter()
            .filter(|(&k, _)| k != id)
            .for_each(|(k, v)| {
                v.send(output.clone()).unwrap_or_else(|err| {
                    println!("ERR: Internal hub error sending message to client {k} with error: {err:#?}");
                });
            });
    }

    pub fn connect(&self, id: Uuid) -> Receiver<Output> {
        let (tx, rx) = sync_channel::<Output>(32);
        let mut map = self.outpost.write().unwrap();
        map.insert(id, tx);
        rx
    }

    fn process_joined(&self, client_id: Uuid, join: &Join) {
        let mut users = self.users.write().unwrap();
        match users.entry(client_id) {
            Entry::Occupied(u) => {
                let user = u.get();
                println!("ERR: Internal hub error, client {client_id} already joined as user {user:#?}");
                self.send_to_user(client_id, Output::Error(OutputErrors::UserAlreadyJoined));
            }
            Entry::Vacant(x) => {
                let user = User::new(client_id, join.name.trim());
                x.insert(user.clone());

                self.send_to_user(
                    client_id,
                    Output::CurrentState(CurrentState::new(
                        user.clone(),
                        users.values().cloned().collect(),
                        self.feed.read().unwrap().clone(),
                    )),
                );

                self.send_to_complement(client_id, &Output::UserJoined(UserActivityTimestamp::new(user)));
            }
        };
    }

    fn process_left(&self, client_id: Uuid) {
        let mut users = self.users.write().unwrap();
        match users.entry(client_id) {
            Entry::Occupied(x) => {
                let user = x.get().clone();
                {
                    let mut map = self.outpost.write().unwrap();
                    map.remove(&client_id);
                }
                x.remove();

                self.send(&Output::UserLeft(UserActivityTimestamp::new(user)));
            }
            Entry::Vacant(_) => {
                println!("WARN: Leaving user of client {client_id} does not exist in hub's user list, not doing anything");
            }
        }
    }

    fn process_post(&self, sender_id: Uuid, post: &Post) {
        let message = Message::new(Uuid::new_v4(), sender_id, post.body.as_str());
        self.feed.write().unwrap().push(message.clone());

        self.send(&Output::Posted(Posted::new(message)));
    }

    async fn process(&self, client_input: ClientInput) {
        match client_input.input {
            Input::Join(join) => self.process_joined(client_input.id, &join),
            Input::Leave => self.process_left(client_input.id),
            Input::Post(post) => self.process_post(client_input.id, &post),
            Input::Error(_) => (),
        };
    }
}
