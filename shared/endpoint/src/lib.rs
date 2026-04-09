use serde::{Deserialize, Serialize};

pub mod user;

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