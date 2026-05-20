use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uchat_domain::{ids::*, Password, Username};
use uchat_domain::ids::UserId;

use crate::{Endpoint, Update};
use crate::post::types::{BookmarkAction, PublicPost};
use crate::user::types::{FollowAction, PublicUserProfile};

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateUser {
    pub username: Username,
    pub password: Password,
}

// impl Endpoint for CreateUser {
//     const URL: &'static str = "/account/create";
// }

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateUserOk {
    pub user_id: UserId,
    pub username: Username,

    pub session_id: SessionId,
    pub session_signature: String,
    pub session_expires: DateTime<Utc>,
}

// 登录
#[derive(Deserialize, Serialize, Clone)]
pub struct Login {
    pub username: Username,
    pub password: Password,
}

// impl Endpoint for Login {
//     const URL: &'static str = "/account/login";
// }

#[derive(Deserialize, Serialize, Clone)]
pub struct LoginOk {
    pub session_signature: String,
    pub session_id: SessionId,
    pub session_expires: DateTime<Utc>,
    
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub profile_image: Option<Url> ,
    pub user_id: UserId
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GetMyProfile;

#[derive(Deserialize, Serialize, Clone)]
pub struct GetMyProfileOk {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub profile_image: Option<Url> ,
    pub user_id: UserId,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateProfile {
    pub display_name: Update<String>,
    pub email: Update<String>,
    pub profile_image: Update<String> ,
    pub password: Update<Password>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateProfileOk {
    pub profile_image: Option<Url>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ViewProfile {
    pub for_user: UserId,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ViewProfileOk {
    pub profile: PublicUserProfile,
    pub posts: Vec<PublicPost>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FollowUser {
    pub user_id: UserId,
    pub action: FollowAction,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FollowUserOk {
    pub status: FollowAction,
}