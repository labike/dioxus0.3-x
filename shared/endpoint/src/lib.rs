use load_dotenv::load_dotenv;
use serde::{Deserialize, Serialize};

pub mod user;
pub mod post;
pub mod trending;

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

load_dotenv!();

pub mod app_url {
    use std::str::FromStr;
    use url::Url;

    pub const API_URL: &str = std::env!("API_URL");
    pub fn domain_and(fragment: &str) -> Url {
        Url::from_str(API_URL).and_then(|url| url.join(fragment)).unwrap()
    }

    pub mod user_content {
        pub const ROOT: &str = "usercontent/";
        pub const IMAGES: &str = "img/";
    }
}

// 公开路由
route!("/account/login" => user::endpoint::Login);
route!("/account/create" => user::endpoint::CreateUser);

// 校验路由
route!("/post/new" => post::endpoint::NewPost);
route!("/posts/bookmark" => post::endpoint::Bookmark);
route!("/posts/react" => post::endpoint::React);
route!("/posts/boost" => post::endpoint::Boost);
route!("/posts/vote" => post::endpoint::Vote);
route!("/posts/trending" => trending::endpoint::TrendingPosts);
