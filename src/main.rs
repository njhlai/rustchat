mod modules;

use std::collections::HashMap;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::modules::data::{Feed, Message, User};
use crate::modules::hub::Hub;
use crate::modules::input::{Input, Join, Post};
use crate::modules::output::{Output, UserJoined};

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

    // let post: Input = serde_json::from_str(r#"{"type": "Post", "payload": { "message": { "id": "dca4d0d7-2d20-4c79-be73-83aa15175643", "sender": "dca4d0d7-2d20-4c79-be73-83aa15175643", "timestamp": "2022-11-29 04:26:49.895795003 UTC", "body": "hello world" } } }"#).unwrap();
    // let post_expected = Input::Post(Post {
    //     message: Message {
    //         id: Uuid::from_str("dca4d0d7-2d20-4c79-be73-83aa15175643").unwrap(),
    //         sender: Uuid::from_str("dca4d0d7-2d20-4c79-be73-83aa15175643").unwrap(),
    //         timestamp: "2022-11-29 04:26:49.895795003 UTC".parse::<DateTime<Utc>>().unwrap(),
    //         body: "hello world".to_string(),
    //     }
    // });
    // assert_eq!(post, post_expected);

    let output: Output = serde_json::from_str(r#"{"type": "UserJoined", "payload": { "user": { "id": "dca4d0d7-2d20-4c79-be73-83aa15175643", "name": "Jack" }, "timestamp": "2022-11-29 04:26:49.895795003 UTC" } }"#).unwrap();
    let output_expected = Output::UserJoined(UserJoined {
        user: User { id: Uuid::from_str("dca4d0d7-2d20-4c79-be73-83aa15175643").unwrap(), name: "Jack".to_string() },
        timestamp: "2022-11-29 04:26:49.895795003 UTC".parse::<DateTime<Utc>>().unwrap()
    });
    assert_eq!(output, output_expected);
    if let Output::UserJoined(x) = output {
        println!("{} joined", x.user.name);
    }

    let hub = Hub::new();

    // user joined
    let user_id_1 = Uuid::new_v4();
    let rx1 = hub.connect(user_id_1);
    hub.process(user_id_1, Input::Join(Join { name: "Jack".to_string() }));
    hub.process(user_id_1, Input::Post(Post { body: "hello world".to_string() }));

    // new user joined
    let user_id_2 = Uuid::new_v4();
    let rx2 = hub.connect(user_id_2);
    hub.process(user_id_2, Input::Join(Join { name: "Jill".to_string() }));
    hub.process(user_id_2, Input::Post(Post { body: "hello Jack".to_string() }));

    // receiver
    println!("{:?}", rx1.try_iter().next().unwrap());
    println!("{:?}", rx1.try_iter().next().unwrap());
    println!("{:?}", rx1.try_iter().next().unwrap());
    println!("{:?}", rx1.try_iter().next().unwrap());

    println!("{:?}", rx2.try_iter().next().unwrap());
    println!("{:?}", rx2.try_iter().next().unwrap());
}