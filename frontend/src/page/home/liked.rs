#![allow(non_snake_case)]
use crate::elements::toaster::use_toaster;
use crate::prelude::{app_bar, use_post_manager, Appbar, AppbarImgButton};
use crate::util::ApiClient;
use crate::{fetch_json, page};
use dioxus::prelude::*;
use dioxus_router::use_router;
use uchat_endpoint::trending::endpoint::{LikePosts, LikePostsOk};

pub fn HomeLiked() -> Element {
    let toaster = use_toaster();
    let api_client = ApiClient::global();
    let post_manager = use_post_manager();
    let router = use_router();

    let _fetch_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future((), |_| async move {
            toaster
                .write()
                .info("Retrieving like posts", chrono::Duration::seconds(3));

            post_manager.write().clear();
            let response = fetch_json!(<LikePostsOk>, api_client, LikePosts);

            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toaster.write().error(
                    format!("Failed to retrive posts: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        })
    };

    // let Posts = post_manager.read().all_to_public();
    let Posts = {
        let posts = post_manager.read().all_to_public();
        if posts.is_empty() {
            let TrendingLink = rsx! {
                a {
                    class: "link",
                    onclick: move |_| {
                        router.navigate_to(page::POSTS_TRENDING);
                    },
                    "trending"
                }
            };

            rsx! {
                div {
                    class: "flex flex-col text-center justify-center h-[calc(100vh_-_var(--navbar-height)_-_var(--appbar-height))]",
                    span {
                        "You haven't liked any posts yet. Check out what's", TrendingLink ", and follow some user"
                    }
                }
            }
        } else {
            rsx! {
                posts.into_iter()
            }
        }
    };

    rsx! {
        Appbar {
            title: "Liked",
            AppbarImgButton {
                click_handler: move |_| (),
                img: "/static/icons/icon-like.svg",
                label: "Liked",
                title: "Show Like Posts",
                disabled: true,
                append_class: app_bar::BUTTON_SELECTED,
            },
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::HOME_BOOKMARKED, None, None),
                img: "/static/icons/icon-bookmark.svg",
                label: "Saved",
                title: "Show Bookmarked Posts",
            },
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::HOME, None, None),
                img: "/static/icons/icon-home.svg",
                label: "Home",
                title: "Go to the gome page",
            },
        },
        Posts
    }
}
