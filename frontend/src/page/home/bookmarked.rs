#![allow(non_snake_case)]
use chrono::Duration;
use dioxus::prelude::*;
use dioxus_router::use_router;
use uchat_endpoint::trending::endpoint::{BookmarkPosts, BookmarkPostsOk, HomePosts, HomePostsOk};
use crate::elements::post::PublicPostEntry;
use crate::elements::toaster::use_toaster;
use crate::{fetch_json, page};
use crate::prelude::{app_bar, use_post_manager, Appbar, AppbarImgButton};
use crate::util::ApiClient;

pub fn HomeBookmarked(cx: Scope) -> Element {
    let toaster = use_toaster(&cx);
    let api_client = ApiClient::global();
    let post_manager = use_post_manager(&cx);
    let router = use_router(&cx);

    let _fetch_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            toaster.write().info("Retrieving posts", chrono::Duration::seconds(3));
            
            post_manager.write().clear();
            let response = fetch_json!(<BookmarkPostsOk>, api_client, BookmarkPosts);

            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toaster.write().error(
                    format!("Failed to retrive posts: {e}"),
                    chrono::Duration::seconds(3),
                )
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
                        "You don't have any bookmarked posts yet. Check out what's", TrendingLink ", and follow some posts"
                    }
                }
            }
        } else {
            rsx! {
                posts.into_iter()
            }
        }
    };

    cx.render(rsx! {
        Appbar {
            title: "Saved",
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::HOME_LIKED, None, None),
                img: "/static/icons/icon-like.svg",
                label: "Liked",
                title: "Show Like Posts",
            },
            AppbarImgButton {
                click_handler: move |_| (),
                img: "/static/icons/icon-bookmark.svg",
                label: "Saved",
                title: "Show Bookmarked Posts",
                disabled: true,
                append_class: app_bar::BUTTON_SELECTED,
            },
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::HOME, None, None),
                img: "/static/icons/icon-home.svg",
                label: "Home",
                title: "Go to the gome page",
            },
        },
        Posts
    })
}