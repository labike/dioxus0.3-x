#![allow(non_snake_case)]

use crate::elements::post::PublicPostEntry;
use crate::fetch_json;
use crate::prelude::*;
use crate::util::ApiClient;
use dioxus::prelude::*;
use uchat_endpoint::trending::endpoint::{TrendingPostOk, TrendingPosts};

#[component]
pub fn Trending() -> Element {
    let api_client = ApiClient::global();
    let navigator = use_navigator();
    let toaster = use_toaster();
    let post_manager = use_post_manager();

    use_future(move || {
        let toaster = toaster.clone();
        let post_manager = post_manager.clone();
        async move {
            toaster
                .write()
                .info("Retrieving trending posts", chrono::Duration::seconds(3));
            let response = fetch_json!(<TrendingPostOk>, api_client, TrendingPosts);

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

    let trending_posts_list = post_manager
        .read()
        .posts
        .iter()
        .map(|(&id, _)| {
            rsx! {
                div {
                    PublicPostEntry {
                        post_id: id
                    }
                }
            }
        })
        .collect::<Vec<Element>>();

    rsx! {
        Appbar {
            title: "Trending Posts",
            AppbarImgButton {
                click_handler: move |_| navigator.go_back(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page",
            },
        },
        {trending_posts_list}
    }
}
