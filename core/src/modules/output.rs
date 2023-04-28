use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::data::{Message, User};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum Output {
    Error(OutputErrors),
    Alive,
    CurrentState(CurrentState),
    UserJoined(UserActivityTimestamp),
    UserLeft(UserActivityTimestamp),
    Posted(Posted),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OutputErrors {
    UserAlreadyJoined,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CurrentState {
    myself: User,
    users: Vec<User>,
    messages: Vec<Message>,
}

impl CurrentState {
    pub fn new(myself: User, users: Vec<User>, messages: Vec<Message>) -> Self {
        CurrentState { myself, users, messages }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserActivityTimestamp {
    user: User,
    timestamp: DateTime<Utc>,
}

impl UserActivityTimestamp {
    pub fn new(user: User) -> Self {
        UserActivityTimestamp { user, timestamp: Utc::now() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Posted {
    message: Message,
}

impl Posted {
    pub fn new(message: Message) -> Self {
        Posted { message }
    }
}
