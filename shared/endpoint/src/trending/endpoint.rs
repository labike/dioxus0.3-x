use serde::{Deserialize, Serialize};
use uchat_domain::ids::PostId;
use crate::Endpoint;
use crate::post::types::{Content, NewPostOptions, PublicPost};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TrendingPosts;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TrendingPostOk {
    pub posts: Vec<PublicPost>
}