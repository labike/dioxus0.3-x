#![allow(non_snake_case)]

use std::collections::hash_map::Iter;
use std::collections::HashMap;
use chrono::{DateTime, Duration, Utc};
use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};

pub fn use_toaster(cx: &ScopeState) -> &UseAtomRef<Toaster> {
    use_atom_ref(cx, crate::app::TOASTER)
}

pub enum ToastKind {
    Error,
    Info,
    Success,
}

pub struct Toast {
    pub message: String,
    pub expires: DateTime<Utc>,
    pub kind: ToastKind
}

#[derive(Default)]
pub struct Toaster {
    toasts: HashMap<usize, Toast>,
    next_id: usize,
}

impl Toaster {
    pub fn increment_id(&mut self) {
        self.next_id += 1;
    }


    pub fn push(&mut self, toast: Toast) {
        self.toasts.insert(self.next_id, toast);
        self.increment_id();
    }

    pub fn remove(&mut self, id: usize) {
        self.toasts.remove(&id);
    }

    pub fn success<T: Into<String>>(&mut self, message: T, duration: Duration) {
        let toast = Toast {
            message: message.into(),
            expires: Utc::now() + duration,
            kind: ToastKind::Success
        };
        self.push(toast);
    }

    pub fn info<T: Into<String>>(&mut self, message: T, duration: Duration) {
        let toast = Toast {
            message: message.into(),
            expires: Utc::now() + duration,
            kind: ToastKind::Info
        };
        self.push(toast);
    }

    pub fn error<T: Into<String>>(&mut self, message: T, duration: Duration) {
        let toast = Toast {
            message: message.into(),
            expires: Utc::now() + duration,
            kind: ToastKind::Error
        };
        self.push(toast);
    }

    pub fn iter(&self) -> Iter<'_, usize, Toast> {
        self.toasts.iter()
    }
}

#[derive(Props)]
pub struct ToastRootProps {
    toaster: &UseAtomRef<Toaster>
}

pub fn ToastRoot(cx: Scope<ToastRootProps>) -> Element {
    let toaster = cx.props.toaster;

    let toasts = &toaster.read();

    let ToastElements = toasts.iter().map(|(&id, toast)| {
        let toast_style = match toast.kind {
            ToastKind::Info => "bg-slate-200 border-slate-300",
            ToastKind::Success => "bg-emerald-200 border-emerald-300",
            ToastKind::Error => "bg-rose-300 border-rose-400",
        };

        rsx! {
            div {
                key: "{id}",
                class: "{toast_style} p-3 cursor-pointer border-solid border rounded",
                onclick: move |_| {
                    toaster.write().remove(id);
                },
                "{toast.message}"
            }
        }
    });

    let total_toasts = &toaster.read().toasts.len();

    let _remove_expired = use_future(cx, (total_toasts,), |(_total_toasts,)| {
        let toaster = toaster.clone();
        async move {
            while !toaster.read().toasts.is_empty() {
                // if total_toasts == 0 {
                //     break;
                // }
                let expired_ids = toaster.read().iter().filter_map(|(&id, toast)| {
                    if Utc::now() > toast.expires {
                        Some(id)
                    } else {
                        None
                    }
                }).collect::<Vec<usize>>();

                expired_ids.iter().for_each(|&id| toaster.write().remove(id));

                gloo_timers::future::TimeoutFuture::new(200_u32).await;
            }
        }
    });

    cx.render(rsx! {
        div { class: "fixed bottom-[var(--navbar-height)] w-screen max-w-[var(--content-max-width)]",
            div { class: "flex flex-col gap-5 px-5 mb-5", ToastElements {} }
        }
    })
}