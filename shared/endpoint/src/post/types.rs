use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uchat_domain::ids::{PostId, UserId};
use uchat_domain::post::{Message, Heading};
use uchat_domain::Username;
use crate::user::types::PublicUserProfile;

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum LikeStatus {
    Dislike,
    Like,
    NoReaction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicPost {
    pub id: PostId,
    pub by_user: PublicUserProfile,
    pub content: Content,
    pub time_posted: DateTime<Utc>,
    pub reply_to: Option<(Username, UserId, PostId)>,
    pub like_status: LikeStatus,
    pub bookmarked: bool,
    pub boosted: bool,
    pub likes: i64,
    pub dislikes: i64,
    pub boosts: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BookmarkAction {
    Add,
    Remove
}