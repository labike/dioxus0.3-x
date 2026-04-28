#![allow(non_snake_case)]

use chrono::Duration;
use dioxus::html::textarea;
use dioxus::prelude::*;
use dioxus_router::use_router;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use uchat_domain::post::{Caption, Heading, Message};
use crate::{async_handler, fetch_json, maybe_class, page, util};
use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
use uchat_endpoint::post::types::{Chat, Image, ImageKind, NewPostOptions};
use crate::prelude::{app_bar, use_toaster, Appbar, AppbarImgButton};
use crate::util::ApiClient;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub caption: String,
    pub image: Option<String>,
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        use uchat_domain::post::Caption;

        if !self.caption.is_empty() && Caption::try_new(&self.caption).is_err() {
            return false;
        }

        if self.image.is_none() {
            return false
        }

        true
    }
}

#[inline_props]
pub fn CaptionInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    use uchat_domain::post::Caption;

    let max_chars = Caption::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().caption.len() > max_chars
    );

    cx.render(rsx! {
        div {
            label {
                r#for: "caption",
                div {
                    class: "flex flex-row justify-between",
                    span {
                        "Caption (optional)"
                    }
                    span {
                        class: "text-right {wrong_len}",
                        "{page_state.read().caption.len()}/{max_chars}",
                    }
                }
            },
            input {
                class: "input-field",
                id: "caption",
                value: "{page_state.read().caption}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.caption = ev.data.value.clone())
                }
            }
        }
    })
}

#[inline_props]
pub fn ImageInput(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let toaster = use_toaster(cx);

    cx.render(rsx! {
        div {
            label {
                r#for: "image-input",
                "Uplaod Image"
            },
            input {
                class: "w-full",
                id: "image-input",
                r#type: "file",
                accept: "image/*",
                oninput: |_| {
                    to_owned![page_state, toaster];
                    async move {
                        use gloo_file::{File, futures::read_as_data_url};
                        use wasm_bindgen::JsCast;

                        let el = util::document().get_element_by_id("image-input").unwrap().unchecked_into::<HtmlInputElement>();
                        let file: File = el.files().unwrap().get(0).unwrap().into();
                        match read_as_data_url(&file).await {
                            Ok(data) => page_state.with_mut(|state| state.image = Some(data)),
                            Err(e) => toaster.write().error(format!("error loading file: {e}"), chrono::Duration::seconds(3))
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn ImagePreview(cx: Scope, page_state: UseRef<PageState>) -> Element {
    let image_data = page_state.read().clone().image;
    let Preview = if let Some(ref image) = image_data {
        rsx! {
            img {
                class: "max-w-[calc(var(--content-max-width)/2)] max-h-[40vh]",
                src: "{image}"
            }
        }
    } else {
        rsx! {
            div {
                "no image uploaded"
            }
        }
    };

    cx.render(rsx! {
        div {
            class: "flex flex-row justify-center",
            Preview
        }
    })
}

pub fn NewImage(cx: Scope) -> Element {
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
                content: Image {
                    caption: {
                        let caption = &page_state.read().caption;
                        if caption.is_empty() {
                            None
                        } else {
                            Some(Caption::try_new(caption).unwrap())
                        }
                    },
                    kind: {
                        let image = &page_state.read().image;
                        ImageKind::DataUrl(image.clone().unwrap())
                    }
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
            title: "New Image",
            AppbarImgButton {
                click_handler: move |_| router.replace_route(page::POST_NEW_CHAT, None, None),
                img: "/static/icons/icon-messages.svg",
                label: "Chat",
                title: "Post a new chat",
            },
            AppbarImgButton {
                click_handler: move |_| (),
                img: "/static/icons/icon-image.svg",
                label: "Image",
                disabled: true,
                title: "Post a new image",
                append_class: app_bar::BUTTON_SELECTED,
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
            ImageInput {
                page_state: page_state.clone(),
            },
            ImagePreview {
                page_state: page_state.clone(),
            }
            CaptionInput {
                page_state: page_state.clone()
            },
            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: !page_state.read().can_submit(),
                "Post"
            }
        }
    })
}