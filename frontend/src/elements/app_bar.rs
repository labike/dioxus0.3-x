#![allow(non_snake_case)]

use crate::prelude::{use_local_profile, use_sidebar};
use dioxus::prelude::*;

pub const BUTTON_SELECTED: &str = "border-b-4 border-slate-600";

#[derive(Props, Clone, PartialEq)]
pub struct AppbarImgButtonProps<'a, F>
where
    F: Fn(MouseEvent),
{
    pub append_class: Option<String>,
    pub click_handler: Option<F>,
    pub disabled: Option<bool>,
    img: String,
    label: String,
    title: Option<String>,
}

#[component]
pub fn AppbarImgButton<F>(props: AppbarImgButtonProps<F>) -> Element
where
    F: Fn(MouseEvent),
{
    let append_class = props.append_class.unwrap_or("");
    rsx! {
        button {
            class: "flex flex-col w-10 h-14 justify-end items-center {append_class}",
            disabled: props.disabled.unwrap_or_default(),
            onclick: |ev| {
                if let Some(callback) = props.click_handler {
                    callback(ev);
                }
            },
            title: props.title.unwrap_or(""),
            img {
                class: "w-6 h-6",
                src: "{props.img}"
            },
            span {
                class: "text-sm",
                "{props.label}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AppbarProps {
    pub title: String,
    pub children: Element,
}

#[component]
pub fn Appbar(props: AppbarProps) -> Element {
    let local_profile = use_local_profile();
    let local_profile = local_profile.read();
    let profile_img_src = local_profile
        .image
        .as_ref()
        .map(|url| url.as_str())
        .unwrap_or_else(|| "");
    let sidebar = use_sidebar();

    rsx! {
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
                    "{props.title}"
                }
                props.children
            }
        }
    }
}
