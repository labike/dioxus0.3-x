use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uchat_domain::ids::{PostId, UserId};
use uchat_domain::post::{Message, Heading};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub heading: Option<Heading>,
    pub message: Message,
}

impl From<Chat> for Content {
    fn from(value: Chat) -> Self {
        Content::Chat(value)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Content {
    Chat(Chat),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewPostOptions {
    pub reply_to: Option<PostId>,
    pub direct_message_to: Option<UserId>,
    pub time_posted: DateTime<Utc>,
}

impl Default for NewPostOptions {
    fn default() -> Self {
        Self {
            reply_to: None,
            direct_message_to: None,
            time_posted: Utc::now(),
        }
    }
}