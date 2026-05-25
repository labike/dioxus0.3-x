use serde::{Deserialize, Serialize};
use crate::post::types::PublicPost;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TrendingPosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TrendingPostOk {
    pub posts: Vec<PublicPost>
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HomePosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HomePostsOk {
    pub posts: Vec<PublicPost>
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LikePosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LikePostsOk {
    pub posts: Vec<PublicPost>
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookmarkPosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookmarkPostsOk {
    pub posts: Vec<PublicPost>
}