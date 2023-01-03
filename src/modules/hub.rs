use std::collections::{HashMap, hash_map::Entry};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::sync::RwLock;

use chrono::Utc;
use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio::sync::mpsc::UnboundedReceiver;
use uuid::Uuid;

use super::data::{Feed, User, Message};
use super::input::{Input, Join, Post};
use super::output::{CurrentState, Output, OutputErrors, UserJoined, Posted};
use super::server::ClientInput;

pub struct Hub {
    pub feed: RwLock<Feed>,
    pub users: RwLock<HashMap<Uuid, User>>,
    pub outpost: RwLock<HashMap<Uuid, SyncSender<Output>>>,
}

impl Hub {
    pub fn new() -> Self {
        Hub {
            feed: Default::default(),
            users: Default::default(),
            outpost: Default::default(),
        }
    }

    pub async fn run(&self, rx: UnboundedReceiver<ClientInput>) {
        UnboundedReceiverStream::new(rx)
            .for_each(|input| self.process(input)).await;

        println!("Hub shutting down");
    }

    pub fn send(&self, output: Output) {
        self.outpost.read().unwrap()
            .values()
            .for_each(|chan|  chan.send(output.clone()).unwrap())
    }

    pub fn send_to_user(&self, id: Uuid, output: Output) {
        self.outpost.read().unwrap()
            .get(&id).unwrap()
            .send(output).unwrap();
    }

    pub fn send_to_complement(&self, id: Uuid, output: Output) {
        self.outpost.read().unwrap()
            .iter().filter(|(&k, _)| k != id)
            .for_each(|(_, v)| v.send(output.clone()).unwrap())
    }

    pub fn connect(&self, id: Uuid) -> Receiver<Output> {
        let (tx, rx) = sync_channel::<Output>(32);
        let mut map = self.outpost.write().unwrap();
        map.insert(id, tx);
        rx
    }

    fn process_joined(&self, client_id: Uuid, join: Join) {
        let mut users = self.users.write().unwrap();
        match users.entry(client_id) {
            Entry::Occupied(_) => {
                self.send_to_user(client_id, Output::Error(OutputErrors::UserAlreadyJoined));
            },
            Entry::Vacant(x) => {
                let user = User {
                    id: client_id,
                    name: join.name.trim().to_string(),
                };
                x.insert(user.clone());

                // send feed to user
                self.send_to_user(client_id, Output::CurrentState(CurrentState {
                    myself: user.clone(),
                    users: users.values().cloned().collect(),
                    messages: self.feed.read().unwrap().messages.clone(),
                }));

                // tell everyone else user joined
                self.send_to_complement(client_id, Output::UserJoined(UserJoined {
                    user: user,
                    timestamp: Utc::now(),
                }));
            },
        };
    }

    fn process_post(&self, sender_id: Uuid, post: Post) {
        let msg = Message {
            id: Uuid::new_v4(),
            sender: sender_id,
            timestamp: Utc::now(),
            body: post.body,
        };
        self.feed.write().unwrap().messages.push(msg.clone());

        self.send(Output::Posted(Posted { message: msg }));
    }

    pub async fn process(&self, client_input: ClientInput) {
        match client_input.input {
            Input::Join(join) => self.process_joined(client_input.id, join),
            Input::Post(post) => self.process_post(client_input.id, post),
        };
    }
}