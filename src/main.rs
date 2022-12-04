use std::{collections::HashMap, str::FromStr};
use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    id: Uuid,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Message {
    id: Uuid,
    sender: Uuid,
    timestamp: DateTime<Utc>,
    body: String,
}

#[derive(Default)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Join {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Post {
    message: Message,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
enum Input {
    Join(Join),
    Post(Post),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
enum Output {
    Error(OutputErrors),
    Alive,
    CurrentState(CurrentState),
    UserJoined(UserJoined),
    UserLeft(UserLeft),
    Posted(Posted),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum OutputErrors {
    Something,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CurrentState {
    myself: User,
    users: Vec<User>,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct UserJoined {
    user: User,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct UserLeft {
    id: Uuid,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Posted {
    message: Message,
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

    let input: Input = serde_json::from_str(r#"{"type": "Join", "payload": { "name": "John" }}"#).unwrap();
    let input_expected = Input::Join(Join { name: "John".to_string() });
    assert_eq!(input, input_expected);

    let post: Input = serde_json::from_str(r#"{"type": "Post", "payload": { "message": { "id": "dca4d0d7-2d20-4c79-be73-83aa15175643", "sender": "dca4d0d7-2d20-4c79-be73-83aa15175643", "timestamp": "2022-11-29 04:26:49.895795003 UTC", "body": "hello world" } } }"#).unwrap();
    let post_expected = Input::Post(Post {
        message: Message {
            id: Uuid::from_str("dca4d0d7-2d20-4c79-be73-83aa15175643").unwrap(),
            sender: Uuid::from_str("dca4d0d7-2d20-4c79-be73-83aa15175643").unwrap(),
            timestamp: "2022-11-29 04:26:49.895795003 UTC".parse::<DateTime<Utc>>().unwrap(),
            body: "hello world".to_string(),
        }
    });
    assert_eq!(post, post_expected);

    let output: Output = serde_json::from_str(r#"{"type": "UserJoined", "payload": { "user": { "id": "dca4d0d7-2d20-4c79-be73-83aa15175643", "name": "Jack" }, "timestamp": "2022-11-29 04:26:49.895795003 UTC" } }"#).unwrap();
    let output_expected = Output::UserJoined(UserJoined {
        user: User { id: Uuid::from_str("dca4d0d7-2d20-4c79-be73-83aa15175643").unwrap(), name: "Jack".to_string() },
        timestamp: "2022-11-29 04:26:49.895795003 UTC".parse::<DateTime<Utc>>().unwrap()
    });
    assert_eq!(output, output_expected);
    if let Output::UserJoined(x) = output {
        println!("{} joined", x.user.name);
    }
}