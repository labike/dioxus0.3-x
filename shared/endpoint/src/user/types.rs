use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uchat_domain::ids::UserId;
use uchat_domain::user::DisplayName;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PublicUserProfile {
    pub id: UserId,
    pub display_name: Option<DisplayName>,
    pub handle: String,
    pub profile_image: Option<Url>,
    pub created_at: DateTime<Utc>,
    pub am_following: bool,
}