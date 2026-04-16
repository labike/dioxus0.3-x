#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use fermi::use_init_atom_root;
use crate::elements::Navbar;
use crate::page;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);
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
            Navbar {}
        }
    })
}
