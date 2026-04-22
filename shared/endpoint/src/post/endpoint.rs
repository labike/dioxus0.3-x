use serde::{Deserialize, Serialize};
use uchat_domain::ids::PostId;
use crate::Endpoint;
use crate::post::types::{BookmarkAction, Content, LikeStatus, NewPostOptions};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPost {
    pub content: Content,
    pub options: NewPostOptions,
}

// impl Endpoint for NewPost {
//     const URL: &'static str = "/post/new";
// }

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NewPostOk {
    pub post_id: PostId,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Bookmark {
    pub post_id: PostId,
    pub action: BookmarkAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookmarkOk {
    pub status: BookmarkAction,
}

impl From<BookmarkAction> for bool {
    fn from(value: BookmarkAction) -> Self {
        match value {
            BookmarkAction::Add => true,
            BookmarkAction::Remove => false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct React {
    pub post_id: PostId,
    pub like_status: LikeStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ReactOk {
    pub like_status: LikeStatus,
    pub likes: i64,
    pub dislikes: i64,
}
