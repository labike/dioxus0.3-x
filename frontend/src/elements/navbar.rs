#![allow(non_snake_case)]

use crate::prelude::*;
use crate::{maybe_class, page};
use dioxus::prelude::*;
use dioxus_router::use_route;

#[inline_props]
pub fn NewPostPopup(hide: UseState<bool>) -> Element {
    let router = use_router();
    let hide_class = maybe_class!("hidden", *hide.get());
    const BUTTON_CLASS: &str =
        "flex gap-4 justify-center items-center w-full h-12 border-y navbar-border-color";

    rsx! {
        div {
            class: "flex flex-col absolute right-0 bottom-[var(--navbar-height)] w-28 items-center {hide_class} navbar-bg-color text-white text-sm",
            div {
                class: BUTTON_CLASS,
                onclick: move |_| router.navigate_to(page::POST_NEW_POLL),
                img {
                    class: "w-[24px] h-[24px]",
                    src: "/static/icons/icon-poll.svg",
                },
                span {"Poll"}
            },
            div {
                class: BUTTON_CLASS,
                onclick: move |_| router.navigate_to(page::POST_NEW_IMAGE),
                img {
                    class: "w-[24px] h-[24px]",
                    src: "/static/icons/icon-image.svg",
                },
                span {"Image"}
            },
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    router.navigate_to(page::POST_NEW_CHAT);
                    hide.set(true);
                },
                img {
                    class: "w-[24px] h-[24px]",
                    src: "/static/icons/icon-messages.svg",
                },
                span {"Chat"}
            }
        }
    }
}

#[derive(Props)]
pub struct NavButtonProps {
    img: &str,
    label: &str,
    onclick: EventHandler<MouseEvent>,
    highlight: Option<bool>,
    children: Element,
}

pub fn NavButton(cx: Scope<NavButtonProps>) -> Element {
    let selected_bgcolor = maybe_class!("bg-slate-500", matches!(cx.props.highlight, Some(true)));

    rsx! {
        button {
            class: "cursor-pointer flex flex-col items-center justify-center h-full {selected_bgcolor}",
            onclick: move |ev| cx.props.onclick.call(ev),
            img {
                class: "invert",
                src: cx.props.img,
                width: "25px",
                height: "25px",
            },
            div {
                class: "text-sm text-white",
                cx.props.label
            },
            &cx.props.children
        }
    }
}
pub fn Navbar() -> Element {
    let hide_new_post_popup = use_state(|| true);
    let _router = use_router();
    let route = use_route();
    let hide_navbar = use_state(|| false);
    let current_route = route.url().path().to_string();

    use_effect((&current_route,), |(current_route,)| {
        to_owned![hide_navbar];
        async move {
            let should_hide = current_route == page::LOGIN || current_route == page::REGISTER;
            hide_navbar.set(should_hide);
        }
    });

    if *hide_navbar.get() {
        return None;
    }

    rsx! {
        nav {
            class: "max-w-[var(--content-max-width)] h-[var(-navbar-height)] fixed bottom-0 left-0 right-0 mx-auto py-2 navbar-bg-color navbar-border-color",
            div {
                class: "grid grid-cols-3 justify-around w-full h-full items-center shadow-inner",
                NavButton {
                    img: "/static/icons/icon-home.svg",
                    label: "Home",
                    onclick: |_| (),
                },
                NavButton {
                    img: "/static/icons/icon-trending.svg",
                    label: "Trending",
                    onclick: |_| (),
                },
                NavButton {
                    img: "/static/icons/icon-new-post.svg",
                    label: "Post",
                    onclick: move |_| {
                        let is_hidden = *hide_new_post_popup.get();
                        hide_new_post_popup.set(!is_hidden);
                    },
                    NewPostPopup {
                        hide: hide_new_post_popup.clone()
                    },
                },
            }
        }
    }
}
