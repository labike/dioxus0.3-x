#![allow(non_snake_case)]

use crate::elements::post::action_bar::Actionbar;
use crate::elements::post::content::Content;
use crate::sync_handler;
use dioxus::prelude::*;
use dioxus_router::Navigator;
use indexmap::IndexMap;
use uchat_domain::ids::{PostId, UserId};
use uchat_endpoint::post::types::PublicPost;

pub mod action_bar;
pub mod content;
pub mod quick_respond;

pub fn use_post_manager() -> Signal<PostManager> {
    use_context::<Signal<PostManager>>()
}

#[derive(Default)]
pub struct PostManager {
    pub posts: IndexMap<PostId, PublicPost>,
}

impl PostManager {
    pub fn update<F>(&mut self, id: PostId, mut update_fn: F) -> bool
    where
        F: FnMut(&mut PublicPost),
    {
        if let Some(post) = self.posts.get_mut(&id) {
            update_fn(post);
            true
        } else {
            false
        }
    }

    pub fn populate<T>(&mut self, posts: T)
    where
        T: Iterator<Item = PublicPost>,
    {
        self.posts.clear();
        for post in posts {
            self.posts.insert(post.id, post);
        }
    }

    pub fn clear(&mut self) {
        self.posts.clear();
    }

    pub fn get(&self, post_id: &PostId) -> Option<&PublicPost> {
        self.posts.get(post_id)
    }

    // IndexMap中shift_remove不会打乱顺序O(n),
    // swap_remove会打乱顺序相当于之前的remove但性能更高O(1)
    pub fn remove(&mut self, post_id: &PostId) {
        self.posts.swap_remove(post_id);
    }

    pub fn all_to_public(&self) -> Vec<Element> {
        self.posts
            .iter()
            .map(|(&id, _)| {
                rsx! {
                    div {
                        PublicPostEntry { post_id: id }
                    }
                }
            })
            .collect()
    }
}

pub fn view_profile_onclick(
    navigator: Navigator,
    user_id: UserId,
) -> impl FnMut(MouseEvent) {
    sync_handler!([navigator], move |_| {
        navigator.push(crate::page::profile_view(user_id));
    })
}

#[component]
pub fn ProfileImage(post: PublicPost) -> Element {
    let navigator = use_navigator();
    let poster_info = &post.by_user;
    let profile_img_src = &poster_info
        .profile_image
        .as_ref()
        .map(|url| url.as_str())
        .unwrap_or_else(|| "");

    rsx! {
        img {
            class: "profile-portrait cursor-pointer",
            onclick: view_profile_onclick(navigator, post.by_user.id),
            src: "{profile_img_src}",
        }
    }
}

#[component]
pub fn Header(post: PublicPost) -> Element {
    let (post_date, post_time) = {
        let date = post.time_posted.format("%Y-%m-%d");
        let time = post.time_posted.format("%H-%M-%S");
        (date, time)
    };

    let display_name = match &post.by_user.display_name {
        Some(name) => name.as_ref(),
        None => "",
    };

    let handle = &post.by_user.handle;

    rsx! {
        div { class: "flex flex-row justify-between",
            div { class: "cursor-pointer", onclick: move |_| (),
                div { "{display_name}" }
                div { class: "font-light", "{handle}" }
            }
            div { class: "text-right",
                div { "{post_date }" }
                div { "{post_time}" }
            }
        }
    }
}

#[component]
pub fn PublicPostEntry(post_id: PostId) -> Element {
    let post_manager = use_post_manager();

    let this_post = {
        let post = post_manager.read().get(&post_id).unwrap().clone();
        use_signal(|| post)
    };

    rsx! {
        div {
            key: "{this_post.read().id.to_string()}",
            class: "grid grid-cols[50px_1fr] gap-1 mb-4",
            ProfileImage { post: this_post.read().clone() }
            div { class: "flex flex-col gap-3",
                Header { post: this_post.read().clone() }
                Content { post: this_post.read().clone() }
                Actionbar { post_id: this_post.read().id }
                hr {}
            }
        }
    }
}
