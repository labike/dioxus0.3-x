#![allow(clippy::redundant_closure_call)]
#![allow(clippy::await_holding_refcell_ref)]
#![allow(clippy::drop_non_drop)]
#![allow(non_snake_case)]

pub mod util;

pub mod app;
pub mod page;
pub mod elements;

use cfg_if::cfg_if;
use crate::util::ApiClient;
use dioxus::LaunchBuilder;

// pub const ROOT_API_URL: &str = "http://127.0.0.1:8070/";
pub const ROOT_API_URL: &str = uchat_endpoint::app_url::API_URL;

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

fn main() {
    init_log();
    ApiClient::init();
    LaunchBuilder::web().launch(app::App);
}

mod prelude {
    pub use crate::util::{async_handler, sync_handler};
    pub use crate::elements::toaster::use_toaster;
    pub use crate::elements::post::use_post_manager;
    pub use crate::elements::app_bar::{self, Appbar, AppbarImgButton};
    
    pub use crate::elements::local_profile::{LocalProfile, use_local_profile};
    pub use crate::elements::sidebar::use_sidebar;
}
