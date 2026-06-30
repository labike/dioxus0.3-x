#![allow(non_snake_case)]

pub mod bookmarked;
pub mod liked;

use crate::elements::toaster::use_toaster;
use crate::prelude::{use_post_manager, Appbar, AppbarImgButton};
use crate::util::ApiClient;
use crate::{fetch_json, page};
use dioxus::prelude::*;
use uchat_endpoint::trending::endpoint::{HomePosts, HomePostsOk};

#[component]
pub fn Home() -> Element {
    let toaster = use_toaster();
    let api_client = ApiClient::global();
    let post_manager = use_post_manager();
    let navigator = use_navigator();

    use_future(move || {
        let mut toaster = toaster.clone();
        let mut post_manager = post_manager.clone();
        async move {
            toaster
                .write()
                .info("Retrieving posts", chrono::Duration::seconds(3));
            let response = fetch_json!(<HomePostsOk>, api_client, HomePosts);

            post_manager.write().clear();

            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toaster.write().error(
                    format!("Failed to retrive posts: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        }
    });

    let Posts = {
        let posts = post_manager.read().all_to_public();
        if posts.is_empty() {
            let trending_link = rsx! {
                a {
                    class: "link",
                    onclick: move |_| {
                        navigator.push(page::Route::Trending {});
                    },
                    "trending"
                }
            };

            rsx! {
                div {
                    class: "flex flex-col text-center justify-center h-[calc(100vh_-_var(--navbar-height)_-_var(--appbar-height))]",
                    span {
                        "check out what's "
                        {trending_link}
                        ", and follow some user"
                    }
                }
            }
        } else {
            rsx! { for post in posts { {post} } }
        }
    };

    rsx! {
        Appbar {
            title: "Home",
            AppbarImgButton {
                click_handler: move || { navigator.replace(page::Route::HomeLiked {}); },
                img: "/static/icons/icon-like.svg",
                label: "Liked",
                title: "Show Like Posts",
            },
            AppbarImgButton {
                click_handler: move || { navigator.replace(page::Route::HomeBookmarked {}); },
                img: "/static/icons/icon-bookmark.svg",
                label: "Saved",
                title: "Show Bookmarked Posts",
            },
            AppbarImgButton {
                click_handler: move || {},
                img: "/static/icons/icon-home.svg",
                label: "Home",
                title: "Go to the gome page",
                disabled: true,
            },
        },
        {Posts}
    }
}
