use std::collections::{HashMap, hash_map::Entry};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::RwLock;

use chrono::Utc;
use uuid::Uuid;

use super::data::{Feed, User};
use super::input::Join;
use super::output::{CurrentState, Output, OutputErrors, UserJoined};

struct Hub {
    feed: RwLock<Feed>,
    users: RwLock<HashMap<Uuid, User>>,
    outpost: RwLock<HashMap<Uuid, Sender<Output>>>,
}

impl Hub {
    pub fn new() -> Self {
        Hub {
            feed: Default::default(),
            users: Default::default(),
            outpost: Default::default(),
        }
    }

    fn send(&self, output: Output) {
        self.outpost.read().unwrap()
            .values()
            .for_each(|chan|  chan.send(output.clone()).unwrap())
    }

    fn send_to_user(&self, id: Uuid, output: Output) {
        self.outpost.read().unwrap()
            .get(&id).unwrap()
            .send(output).unwrap();
    }

    fn send_to_complement(&self, id: Uuid, output: Output) {
        self.outpost.read().unwrap()
            .iter().filter(|(&k, _)| k != id)
            .for_each(|(_, v)| v.send(output.clone()).unwrap())
    }

    fn connect(&self, id: Uuid) -> Receiver<Output> {
        let (tx, rx) = channel::<Output>();
        let mut map = self.outpost.write().unwrap();
        map.insert(id, tx);
        rx
    }

    fn process_joined(&self, id: Uuid, join: Join) {
        match self.users.write().unwrap().entry(id) {
            Entry::Occupied(_) => {
                self.send_to_user(id, Output::Error(OutputErrors::UserAlreadyJoined));
            },
            Entry::Vacant(x) => {
                let user = User {
                    id: id,
                    name: join.name.trim().to_string(),
                };
                x.insert(user.clone());

                // send feed to user
                self.send_to_user(id, Output::CurrentState(CurrentState {
                    myself: user.clone(),
                    users: self.users.read().unwrap().values().cloned().collect(),
                    messages: self.feed.read().unwrap().messages.clone(),
                }));

                // tell everyone else user joined
                self.send_to_complement(id, Output::UserJoined(UserJoined {
                    user: user,
                    timestamp: Utc::now(),
                }));
            },
        };
    }
}