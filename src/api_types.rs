use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserMetadata {
    pub id: i32,
    pub username: String,
    pub discriminator: i32,
    pub last_check_in: DateTime<Utc>,
    pub picture: String,
    pub account_creation: DateTime<Utc>,
    pub description: String
}

#[derive(Debug, Serialize)]
pub struct NewMessage {
    channel_id: i32,
    author_id: i32,
    content: String
}

impl NewMessage {
    pub fn new(channel_id: i32, author_id: i32, content: String) -> Self {
        Self { channel_id, author_id, content }
    }
}

#[derive(Debug, Deserialize)]
pub struct Message {
    id: i32,
    author_id: i32,
    channel_id: i32,
    content: String,
    creation_date: DateTime<Utc>,
}

