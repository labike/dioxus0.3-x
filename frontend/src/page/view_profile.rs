#![allow(non_snake_case)]

use crate::fetch_json;
use crate::prelude::*;
use crate::util::ApiClient;
use dioxus::prelude::*;
use uchat_domain::ids::UserId;
use uchat_endpoint::user::endpoint::{FollowUser, FollowUserOk};
use uchat_endpoint::user::types::FollowAction;

#[component]
pub fn ViewProfile(user: UserId) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster();
    let navigator = use_navigator();
    let post_manager = use_post_manager();
    let profile = use_signal(|| None);
    let local_profile = use_local_profile();

    use_future(move || {
        let mut post_manager = post_manager.clone();
        let mut profile = profile.clone();
        let mut toaster = toaster.clone();
        async move {
            post_manager.write().clear();
            use uchat_endpoint::user::endpoint::{ViewProfile, ViewProfileOk};
            let request_data = ViewProfile { for_user: user };
            post_manager.write().clear();
            let response = fetch_json!(<ViewProfileOk>, api_client, request_data);
            match response {
                Ok(res) => {
                    profile.set(Some(res.profile));
                    post_manager.write().populate(res.posts.into_iter());
                }
                Err(e) => toaster.write().error(
                    format!("Failed to retrieve posts: {e}"),
                    chrono::Duration::seconds(3),
                ),
            }
        }
    });

    let follow_onclick = async_handler!([api_client, toaster, profile], move |_| async move {
        let am_following = match profile.read().as_ref() {
            Some(profile) => profile.am_following,
            None => false,
        };

        let request_data = FollowUser {
            action: match am_following {
                true => FollowAction::UnFollow,
                false => FollowAction::Follow,
            },
            user_id: user,
        };

        match fetch_json!(<FollowUserOk>, api_client, request_data) {
            Ok(res) => {
                profile.with_mut(|profile| {
                    profile.as_mut().map(|p| p.am_following = res.status.into())
                });
            }
            Err(e) => toaster.write().error(
                format!("Failed to update follow status: {}", e),
                chrono::Duration::seconds(3),
            ),
        }
    });

    let profile_section = {
        match profile() {
            Some(profile) => {
                let display_name = profile
                    .display_name
                    .map(|name| name.into_inner())
                    .unwrap_or_else(|| "(None)".to_string());
                let profile_image = profile
                    .profile_image
                    .map(|url| url.to_string())
                    .unwrap_or_else(|| "".to_string());
                let follow_button_text = match profile.am_following {
                    true => "Unfollow",
                    false => "Follow",
                };
                let follow_button = local_profile.read().user_id.and_then(|id| {
                    if id == profile.id {
                        None
                    } else {
                        Some(rsx! {
                            button {
                                class: "btn",
                                onclick: follow_onclick,
                                "{follow_button_text}"
                            }
                        })
                    }
                });

                rsx! {
                    div {
                        class: "flex flex-col gap-3",
                        div {
                            class: "flex flex-row justify-center",
                            img {
                                class: "profile-portrait-lg",
                                src: "{profile_image}",
                            }
                        },
                        div {
                            "Handle: {profile.handle}",
                        },
                        div {
                            "Name: {display_name}"
                        },
                        {follow_button}
                    }
                }
            }
            None => rsx! {
                "Loading profile..."
            },
        }
    };

    let posts = post_manager.read().all_to_public();

    rsx! {
        Appbar {
            title: "View Profile",
            AppbarImgButton {
                click_handler: move || navigator.go_back(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page",
            },
        },
        {profile_section},
        div {
            class: "font-bold text-center my-6",
            "Posts"
        },
        hr {
            class: "h-px my-6 bg-gray-200 border-0"
        },
        for post in posts { {post} }
    }
}
