#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "home page"
        }
    })
}