pub mod register;
pub mod login;
pub mod home;
pub mod new_post;
pub mod trending;
pub mod edit_profile;
pub mod view_profile;

pub use register::Register;
pub use login::Login;
pub use home::{Home, bookmarked::HomeBookmarked, liked::HomeLiked};
pub use new_post::*;
pub use trending::Trending;
pub use edit_profile::EditProfile;
pub use view_profile::ViewProfile;

pub use route::*;

pub mod route {
    use uchat_domain::ids::UserId;

    pub const REGISTER: &str = "/account/register";
    pub const LOGIN: &str = "/account/login";
    pub const HOME: &str = "/home";
    pub const HOME_BOOKMARKED: &str = "/home/bookmarked";
    pub const HOME_LIKED: &str = "/home/liked";
    pub const POST_NEW_CHAT: &str = "/post/new_chat";
    pub const POST_NEW_IMAGE: &str = "/post/new_image";
    pub const POST_NEW_POLL: &str = "/post/new_poll";
    pub const POSTS_TRENDING: &str = "/posts/trending";
    pub const EDIT_PROFILE: &str = "/profile/edit";
    pub const PROFILE_VIEW: &str = "/profile/view/:user";

    pub fn profile_view(user_id: UserId) -> String {
        PROFILE_VIEW.replace(":user", &user_id.to_string())
    }
}