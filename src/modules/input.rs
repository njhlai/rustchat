use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct ClientInput {
    pub id: Uuid,
    pub input: Input,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum Input {
    Join(Join),
    Post(Post),
    Error(InputErrors),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Join {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum InputErrors {
    InputParseError,
}