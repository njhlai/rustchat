use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: Uuid,
    pub sender: Uuid,
    pub timestamp: DateTime<Utc>,
    pub body: String,
}

#[derive(Default)]
pub struct Feed {
    messages: Vec<Message>,
}

impl Feed {
    pub fn clone(&self) -> Vec<Message> {
        self.messages.clone()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }

    pub fn push(&mut self, message: Message) {
        let index = self.messages.partition_point(|x| x.timestamp < message.timestamp);

        self.messages.insert(index, message);
    }
}