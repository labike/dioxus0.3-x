#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{use_router, Route, Router};
use fermi::{use_init_atom_root, AtomRef};
use uchat_endpoint::user::endpoint::{GetMyProfile, GetMyProfileOk};
use crate::elements::Navbar;
use crate::elements::post::PostManager;
use crate::elements::toaster::{ToastRoot, Toaster};
use crate::{fetch_json, page};
use crate::elements::sidebar::{Sidebar, SidebarManager};
use crate::prelude::{use_local_profile, use_toaster, LocalProfile};
use crate::util::ApiClient;

pub static TOASTER: AtomRef<Toaster> = |_| Toaster::default();
pub static POSTMANAGER: AtomRef<PostManager> = |_| PostManager::default();
pub static LOCAL_PROFILE: AtomRef<LocalProfile> = |_| LocalProfile::default();
pub static SIDEBAR: AtomRef<SidebarManager> = |_| SidebarManager::default();


pub fn Init(cx: Scope) -> Element {
    let local_profile = use_local_profile(cx);
    let toaster = use_toaster(cx);
    let api_client = ApiClient::global();
    let router = use_router(cx);

    let _fetch_local_profile = {
        to_owned![api_client, toaster, router, local_profile];
        use_future(cx, (), |_| async move {
            let response = fetch_json!(<GetMyProfileOk>, api_client, GetMyProfile);

            match response {
                Ok(res) => {
                    local_profile.write().image = res.profile_image;
                    local_profile.write().user_id = Some(res.user_id);
                },
                Err(_e) => {
                    toaster.write().error(
                        "please login or create account.",
                        chrono::Duration::seconds(3),
                    );
                    router.navigate_to(page::LOGIN)
                }
            }
        })
    };
    None
}

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let toaster = use_toaster(cx);
    cx.render(rsx!{
        Router {
            Init{},
            Sidebar {},
            main {
                class: "max-w-[var(--content-max-width)] min-w[var(--content-min-width)] mt-[var(--appbar-height)] mb-[var(--navbar-height)] max-auto p-4",
                Route {
                    to: page::route::REGISTER,
                    page::Register {}
                },
                Route {
                    to: page::route::LOGIN,
                    page::Login {}
                },
                Route {
                    to: page::route::HOME,
                    page::Home {}
                },
                Route {
                    to: page::route::HOME_LIKED,
                    page::HomeLiked {}
                },
                Route {
                    to: page::route::HOME_BOOKMARKED,
                    page::HomeBookmarked {}
                },
                Route {
                    to: page::route::POST_NEW_CHAT,
                    page::NewChat {}
                },
                Route {
                    to: page::route::POST_NEW_IMAGE,
                    page::NewImage {}
                },
                Route {
                    to: page::route::POSTS_TRENDING,
                    page::Trending {}
                },
                Route {
                    to: page::route::POST_NEW_POLL,
                    page::NewPoll {}
                },
                Route {
                    to: page::route::EDIT_PROFILE,
                    page::EditProfile {}
                }
            },
            ToastRoot {
                toaster: toaster
            },
            Navbar {}
        }
    })
}
