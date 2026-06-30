#![allow(non_snake_case)]

use crate::prelude::*;
use crate::{maybe_class, page};
use dioxus::prelude::*;

#[component]
pub fn NewPostPopup(hidden: Signal<bool>) -> Element {
    let navigator = use_navigator();
    let hide_class = maybe_class!("hidden", *hidden.read());
    const BUTTON_CLASS: &str =
        "flex gap-4 justify-center items-center w-full h-12 border-y navbar-border-color";

    rsx! {
        div {
            class: "flex flex-col absolute right-0 bottom-[var(--navbar-height)] w-28 items-center {hide_class} navbar-bg-color text-white text-sm",
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    navigator.push(page::Route::NewPoll {});
                },
                img {
                    class: "w-[24px] h-[24px]",
                    src: "/static/icons/icon-poll.svg",
                },
                span { "Poll" }
            },
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    navigator.push(page::Route::NewImage {});
                },
                img {
                    class: "w-[24px] h-[24px]",
                    src: "/static/icons/icon-image.svg",
                },
                span { "Image" }
            },
            div {
                class: BUTTON_CLASS,
                onclick: move |_| {
                    navigator.push(page::Route::NewChat {});
                    hidden.set(true);
                },
                img {
                    class: "w-[24px] h-[24px]",
                    src: "/static/icons/icon-messages.svg",
                },
                span { "Chat" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NavButtonProps {
    img: &'static str,
    label: &'static str,
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    highlight: bool,
    children: Element,
}

#[component]
pub fn NavButton(props: NavButtonProps) -> Element {
    let selected_bgcolor = maybe_class!("bg-slate-500", props.highlight);

    rsx! {
        button {
            class: "cursor-pointer flex flex-col items-center justify-center h-full {selected_bgcolor}",
            onclick: move |ev| props.onclick.call(ev),
            img {
                class: "invert",
                src: props.img,
                width: "25px",
                height: "25px",
            }
            div {
                class: "text-sm text-white",
                {props.label}
            }
            {props.children}
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    let mut hide_new_post_popup = use_signal(|| true);
    let route: page::Route = use_route();
    let hide_navbar = matches!(route, page::Route::Login {} | page::Route::Register {});

    if hide_navbar {
        return rsx! {};
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
                    children: rsx! {}
                }
                NavButton {
                    img: "/static/icons/icon-trending.svg",
                    label: "Trending",
                    onclick: |_| (),
                    children: rsx! {}
                }
                NavButton {
                    img: "/static/icons/icon-new-post.svg",
                    label: "Post",
                    onclick: move |_| {
                        let is_hidden = *hide_new_post_popup.read();
                        hide_new_post_popup.set(!is_hidden);
                    },
                    children: rsx! {
                        NewPostPopup {
                            hidden: hide_new_post_popup
                        }
                    }
                }
            }
        }
    }
}
