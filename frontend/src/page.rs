pub mod register;
pub mod login;
pub mod home;
pub mod new_post;
pub mod trending;

pub use register::Register;
pub use login::Login;
pub use home::Home;
pub use new_post::*;
pub use trending::Trending;

pub use route::*;

pub mod route {
    pub const REGISTER: &str = "/account/register";
    pub const LOGIN: &str = "/account/login";
    pub const HOME: &str = "/home";
    pub const POST_NEW_CHAT: &str = "/post/new_chat";
    pub const POST_NEW_IMAGE: &str = "/post/new_image";
    pub const POSTS_TRENDING: &str = "/posts/trending";
}