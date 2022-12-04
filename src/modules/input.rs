use serde::{Deserialize, Serialize};

use super::data::Message;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum Input {
    Join(Join),
    Post(Post),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Join {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub message: Message,
}