#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::prelude::{use_local_profile, use_sidebar};

pub const BUTTON_SELECTED: &str = "border-b-4 border-slate-600";

#[derive(Props)]
pub struct AppbarImgButtonProps<F>
where F: Fn(Event<MouseData>) {
    pub append_class: Option<&str>,
    pub click_handler: Option<F>,
    pub disabled: Option<bool>,
    img: &str,
    label: &str,
    title: Option<&str>,
}

pub fn AppbarImgButton<F>(
    cx: Scope<AppbarImgButtonProps<F>>
) -> Element
where F: Fn(Event<MouseData>) {
    let append_class = cx.props.append_class.unwrap_or("");
    cx.render(rsx! {
        button {
            class: "flex flex-col w-10 h-14 justify-end items-center {append_class}",
            disabled: cx.props.disabled.unwrap_or_default(),
            onclick: |ev| {
                if let Some(callback) = &cx.props.click_handler {
                    callback(ev);
                }
            },
            title: cx.props.title.unwrap_or(""),
            img {
                class: "w-6 h-6",
                src: "{cx.props.img}"
            },
            span {
                class: "text-sm",
                "{cx.props.label}"
            }
        }
    })
}

#[derive(Props)]
pub struct AppbarProps {
    pub title: &str,
    pub children: Element,
}

pub fn Appbar(cx: Scope<AppbarProps>) -> Element {
    let local_profile = use_local_profile(cx);
    let local_profile = local_profile.read();
    let profile_img_src = local_profile.image.as_ref().map(|url| url.as_str()).unwrap_or_else(|| "");
    let sidebar= use_sidebar(cx);

    cx.render(rsx! {
        div {
            class: "max-w-[var(--content-max-width)] h-[var(--appbar-height)] fixed top-0 right-0 left-0 max-auto z-50 bg-slate-200",
            div {
                class: "flex flex-row gap-3 items-center w-full pr-5 h-full",
                div {
                    class: "cursor-pointer",
                    onclick: move |_| sidebar.write().open(),
                    img {
                        class: "profile-portrait",
                        src: "{profile_img_src}"
                    },
                },
                div {
                    class: "text-xl font-bold mr-auto",
                    "{cx.props.title}"
                }
                &cx.props.children
            }
        }
    })
}