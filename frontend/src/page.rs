use crate::page::view_profile::ViewProfile;
use dioxus::dioxus_core;
use dioxus::prelude::{dioxus_signals, rsx, GlobalSignal, VNode};
pub mod register;
pub mod login;
pub mod home;
pub mod new_post;
pub mod trending;
pub mod edit_profile;
pub mod view_profile;

use dioxus_router::Routable;
use uchat_domain::ids::UserId;

pub use edit_profile::EditProfile;
pub use home::{bookmarked::HomeBookmarked, liked::HomeLiked, Home};
pub use login::Login;
pub use new_post::*;
pub use register::Register;
pub use trending::Trending;

#[derive(Routable, Clone, Debug, PartialEq)]
pub enum Route {
    #[layout(crate::app::Layout)]
        #[route("/account/register")]
        Register {},
        #[route("/account/login")]
        Login {},
        #[route("/home")]
        Home {},
        #[route("/home/bookmarked")]
        HomeBookmarked {},
        #[route("/home/liked")]
        HomeLiked {},
        #[route("/post/new_chat")]
        NewChat {},
        #[route("/post/new_image")]
        NewImage {},
        #[route("/post/new_poll")]
        NewPoll {},
        #[route("/posts/trending")]
        Trending {},
        #[route("/profile/edit")]
        EditProfile {},
        #[route("/profile/view/:user")]
        ViewProfile { user: UserId },
}

pub fn profile_view(user_id: UserId) -> Route {
    Route::ViewProfile { user: user_id }
}
