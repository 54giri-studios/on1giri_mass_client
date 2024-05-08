use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Channel {
    /// It's globally unique id
    pub id: i32,
    /// The guild's id that it belongs to
    /// Must refer to an actual [crate::Guild]
    pub guild_id: i32,
    /// It's display name
    pub name: String,
    /// The kind of the channel
    /// Must refer to an actual [crate::ChannelKind]
    pub kind: String
}


#[derive(Debug, Deserialize)]
pub struct PopulatedMessage {
    id: i32,
    channel: Channel,
    author: UserMetadata,
    content: String,
    creation_date: DateTime<Utc>,
}

