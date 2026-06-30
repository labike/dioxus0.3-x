#![allow(non_snake_case)]

use crate::elements::post::PostManager;
use crate::elements::sidebar::{Sidebar, SidebarManager};
use crate::elements::toaster::{ToastRoot, Toaster};
use crate::elements::Navbar;
use crate::prelude::{use_local_profile, use_toaster, LocalProfile};
use crate::util::ApiClient;
use crate::{fetch_json, page};
use dioxus::prelude::*;
use dioxus_router::{use_navigator, Outlet, Router};
use uchat_endpoint::user::endpoint::{GetMyProfile, GetMyProfileOk};

#[component]
pub fn Init() -> Element {
    let local_profile = use_local_profile();
    let toaster = use_toaster();
    let api_client = ApiClient::global();
    let navigator = use_navigator();

    use_future(move || {
        let mut local_profile = local_profile.clone();
        let mut toaster = toaster.clone();
        async move {
            let response = fetch_json!(<GetMyProfileOk>, api_client, GetMyProfile);

            match response {
                Ok(res) => {
                    local_profile.write().image = res.profile_image;
                    local_profile.write().user_id = Some(res.user_id);
                }
                Err(_e) => {
                    toaster
                        .write()
                        .error("please login or create account.", chrono::Duration::seconds(3));
                    navigator.push(page::Route::Login {});
                }
            }
        }
    });

    rsx! {}
}

#[component]
pub fn Layout() -> Element {
    rsx! {
        Init {}
        Sidebar {}
        main {
            class: "max-w-[var(--content-max-width)] min-w[var(--content-min-width)] mt-[var(--appbar-height)] mb-[var(--navbar-height)] max-auto p-4",
            Outlet::<page::Route> {}
        }
        ToastRoot {}
        Navbar {}
    }
}

#[component]
pub fn App() -> Element {
    let toaster = use_signal(Toaster::default);
    let post_manager = use_signal(PostManager::default);
    let local_profile = use_signal(LocalProfile::default);
    let sidebar = use_signal(SidebarManager::default);

    provide_context(toaster);
    provide_context(post_manager);
    provide_context(local_profile);
    provide_context(sidebar);

    rsx! {
        Router::<page::Route> {}
    }
}
