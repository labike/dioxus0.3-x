use serde::{Deserialize, Serialize};

pub mod user;
pub mod post;

pub trait Endpoint {
    const URL: &'static str;
    fn url(&self) -> &'static str {
        Self::URL
    }
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[error("{msg}")]
pub struct RequestFailed {
    pub msg: String,
}

macro_rules! route {
    ($url:literal => $request_type:ty) => {
        impl Endpoint for $request_type {
            const URL: &'static str = $url;
        }
    }
}

// 公开路由
route!("/account/login" => user::endpoint::Login);
route!("/account/create" => user::endpoint::CreateUser);

// 校验路由
route!("/post/new" => post::endpoint::NewPost);