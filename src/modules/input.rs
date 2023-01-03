use serde::{Deserialize, Serialize};

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