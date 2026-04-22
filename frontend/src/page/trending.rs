#![allow(non_snake_case)]

use dioxus::prelude::*;
use uchat_endpoint::trending::endpoint::{TrendingPostOk, TrendingPosts};
use crate::elements::post::PublicPostEntry;
use crate::fetch_json;
use crate::prelude::*;
use crate::util::ApiClient;

pub fn Trending(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let toaster = use_toaster(cx);
    let post_manager = use_post_manager(cx);

    let _fetch_trending_posts = {
        to_owned![api_client, toaster, post_manager];
        use_future(cx, (), |_| async move {
            toaster.write().info("Retrieving trending posts", chrono::Duration::seconds(3));
            let response = fetch_json!(<TrendingPostOk>, api_client, TrendingPosts);

            match response {
                Ok(res) => post_manager.write().populate(res.posts.into_iter()),
                Err(e) => toaster.write().error(
                    format!("Failed to retrive posts: {e}"),
                    chrono::Duration::seconds(3),
                )
            }
        })
    };

    let TrendingPostsList = post_manager.read().posts.iter().map(|(&id, _)| {
        rsx! {
            div {
                PublicPostEntry {
                    post_id: id
                }
            }
        }
    }).collect::<Vec<LazyNodes>>();

    cx.render(rsx! {
        TrendingPostsList.into_iter()
    })
}