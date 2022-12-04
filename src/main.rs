use std::{collections::HashMap, str::FromStr};
use chrono::{DateTime,Utc};
use uuid::Uuid;

struct User {
    id: Uuid,
    name: String,
}

struct Message {
    id: Uuid,
    sender: Uuid,
    timestamp: DateTime<Utc>,
    body: String,
}

struct Feed {
    messages: Vec<Message>,
}

impl Feed {
    pub fn push(&mut self, message: Message) {
        let index = self.messages.partition_point(|x| x.timestamp < message.timestamp);

        self.messages.insert(index, message);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }
}

fn main() {
    let mut feed: Feed = Default::default();

    let test_user = User { id: Uuid::new_v4(), name: "Test User".into() };
    let mut users = HashMap::new();
    let test_id = test_user.id;
    users.insert(test_user.id, test_user);

    let test_message = Message {
        id: Uuid::new_v4(),
        sender: test_id,
        timestamp: Utc::now(),
        body: "This is a test message".into()
    };

    feed.push(test_message);

    for msg in feed.iter() {
        let user = users.get(&msg.sender).unwrap();
        println!("{}: {}, sent by {} {}", msg.timestamp, msg.body, user.name, user.id);
    }
}