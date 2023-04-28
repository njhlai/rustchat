use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    id: Uuid,
    name: String,
}

impl User {
    pub fn new(id: Uuid, name: &str) -> Self {
        User { id, name: String::from(name) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message {
    id: Uuid,
    sender: Uuid,
    timestamp: DateTime<Utc>,
    body: String,
}

impl Message {
    pub fn new(id: Uuid, sender: Uuid, body: &str) -> Self {
        Message { id, sender, timestamp: Utc::now(), body: String::from(body) }
    }
}

#[derive(Default)]
pub struct Feed {
    messages: Vec<Message>,
}

impl Feed {
    pub fn clone(&self) -> Vec<Message> {
        self.messages.clone()
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }

    pub fn push(&mut self, message: Message) {
        let index = self.messages.partition_point(|x| x.timestamp < message.timestamp);

        self.messages.insert(index, message);
    }
}
