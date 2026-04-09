use serde::{Deserialize, Serialize};
use uchat_domain::{Password, Username};
use uchat_domain::ids::UserId;

use crate::Endpoint;

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateUser {
    pub username: Username,
    pub password: Password,
}

impl Endpoint for CreateUser {
    const URL: &'static str = "/account/create";
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateUserOk {
    pub user_id: UserId,
    pub username: Username,
}