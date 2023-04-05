use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::data::{Message, User};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum Output {
    Error(OutputErrors),
    Alive,
    CurrentState(CurrentState),
    UserJoined(UserJoined),
    UserLeft(UserLeft),
    Posted(Posted),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OutputErrors {
    UserAlreadyJoined,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CurrentState {
    pub myself: User,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserJoined {
    pub user: User,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserLeft {
    pub user: User,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Posted {
    pub message: Message,
}