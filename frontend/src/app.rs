#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::{use_init_atom_root, AtomRef};
use crate::elements::Navbar;
use crate::elements::toaster::{ToastRoot, Toaster};
use crate::page;
use crate::prelude::use_toaster;

pub static TOASTER: AtomRef<Toaster> = |_| Toaster::default();

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let toaster = use_toaster(cx);
    cx.render(rsx!{
        Router {
            Route {
                to: page::route::REGISTER,
                page::Register {}
            },
            Route {
                to: page::route::LOGIN,
                page::Login {}
            },
            Route {
                to: page::route::HOME,
                page::Home {}
            },
            Route {
                to: page::route::POST_NEW_CHAT,
                page::NewChat {}
            },
            ToastRoot {
                toaster: toaster
            },
            Navbar {}
        }
    })
}
