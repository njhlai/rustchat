use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct ClientInput {
    pub id: Uuid,
    pub input: Input,
}

impl ClientInput {
    pub fn new(id: Uuid, input: Input) -> Self {
        ClientInput { id, input }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum Input {
    Join(Join),
    Leave,
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
