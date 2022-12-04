use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::data::{Message,User};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum Output {
    Error(OutputErrors),
    Alive,
    CurrentState(CurrentState),
    UserJoined(UserJoined),
    UserLeft(UserLeft),
    Posted(Posted),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OutputErrors {
    Something,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CurrentState {
    pub myself: User,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserJoined {
    pub user: User,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserLeft {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Posted {
    pub message: Message,
}