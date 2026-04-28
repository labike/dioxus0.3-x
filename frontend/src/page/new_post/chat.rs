#![allow(non_snake_case)]

use chrono::Duration;
use dioxus::html::textarea;
use dioxus::prelude::*;
use dioxus_router::use_router;
use serde::{Deserialize, Serialize};
use uchat_domain::post::{Heading, Message};
use crate::{async_handler, fetch_json, maybe_class, page};
use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
use uchat_endpoint::post::types::{Chat, NewPostOptions};
use crate::elements::app_bar::AppbarImgButton;
use crate::prelude::{app_bar, use_toaster, Appbar};
use crate::util::ApiClient;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub message: String,
    pub heading: String,
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        use uchat_domain::post::{Heading, Message};

        if Message::try_new(&self.message).is_err() {
            return false;
        }

        if !&self.heading.is_empty() && Heading::try_new(&self.heading).is_err() {
            return false;
        }

        true
    }
}

#[inline_props]
pub fn MessageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::post::Message;

    let max_chars = Message::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().message.len() > max_chars || page_state.read().message.is_empty()
    );

    cx.render(rsx! {
        div {
            label {
                r#for: "message",
                div {
                    class: "flex flex-row justify-between",
                    span {
                        "Message"
                    }
                    span {
                        class: "text-right {wrong_len}",
                        "{page_state.read().message.len()}/{max_chars}",
                    }
                }
            },
            textarea {
                class: "input-field",
                id: "message",
                rows: 5,
                value: "{page_state.read().message}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.message = ev.data.value.clone())
                }
            }
        }
    })
}

#[inline_props]
pub fn HeadingInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::post::Heading;

    let max_chars = Heading::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().heading.len() > max_chars || page_state.read().heading.is_empty()
    );

    cx.render(rsx! {
        div {
            label {
                r#for: "heading",
                div {
                    class: "flex flex-row justify-between",
                    span {
                        "Heading"
                    }
                    span {
                        class: "text-right {wrong_len}",
                        "{page_state.read().heading.len()}/{max_chars}",
                    }
                }
            },
            input {
                class: "input-field",
                id: "heading",
                value: "{page_state.read().heading}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.heading = ev.data.value.clone())
                }
            }
        }
    })
}

pub fn NewChat(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    let router = use_router(cx);
    let toaster = use_toaster(cx);

    let page_state = use_ref(&cx, PageState::default);

    let submit_btn_style = maybe_class!("btn-disabled", !page_state.read().can_submit());

    let form_onsubmit = async_handler!(
        &cx,
        [api_client, page_state, toaster, router],
        move |_| async move {
            let request_data = NewPost {
                content: Chat {
                    heading: {
                        let heading = &page_state.read().heading;
                        if heading.is_empty() {
                            None
                        } else {
                            Some(Heading::try_new(heading).unwrap())
                        }
                    },
                    message: Message::try_new(&page_state.read().message).unwrap(),
                }.into(),
                options: NewPostOptions::default(),
            };

            let response = fetch_json!(<NewPostOk>, api_client, request_data);

            match response {
                Ok(_) => {
                    toaster.write().success("Posted!", Duration::seconds(3));
                    // 禁止返回post
                    router.replace_route(page::HOME, None, None);
                },
                Err(e) => {
                    toaster.write().error(format!("Post failed: {e}"), Duration::seconds(3));
                },
            }
        }
    );

    cx.render(rsx! {
        Appbar {
            title: "New Chat",
            AppbarImgButton {
                click_handler: move |_| (),
                img: "/static/icons/icon-message.svg",
                label: "Chat",
                title: "Post a new chat",
                disabled: true,
                append_class: app_bar::BUTTON_SELECTED
            },
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_IMAGE, None, None),
                img: "/static/icons/icon-image.svg",
                label: "Image",
                title: "Post a new image",
            }
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_POLL, None, None),
                img: "/static/icons/icon-poll.svg",
                label: "Poll",
                title: "Post a new poll",
            },
            AppbarImgButton {
                click_handler: move |_| router.pop_route(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page",
            },
        },
        form {
            class: "flex flex-col gap-4",
            onsubmit: form_onsubmit,
            prevent_default: "onsubmit",
            MessageInput {
                page_state: page_state.clone()
            },
            HeadingInput {
                page_state: page_state.clone()
            }
            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: !page_state.read().can_submit(),
                "Post"
            }
        }
    })
}