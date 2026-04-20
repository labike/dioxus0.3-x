#![allow(non_snake_case)]

use chrono::Duration;
use dioxus::prelude::*;
use crate::elements::toaster::use_toaster;

pub fn Home(cx: Scope) -> Element {
    let toaster = use_toaster(&cx);
    cx.render(rsx! {
        h1 {
            "home page"
        }
        button {
            onclick: move |_| {
                toaster.write().success("success", Duration::seconds(5));
                toaster.write().success("info", Duration::seconds(5));
                toaster.write().success("error", Duration::seconds(5));
            },
            "toast"
        }
    })
}