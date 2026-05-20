#![allow(non_snake_case)]

use dioxus::prelude::*;
use uchat_domain::UserFacingError;
use crate::elements::keyed_notification_box::{KeyedNotificationBox, KeyedNotifications};
use crate::{fetch_json, maybe_class, page};
use crate::prelude::*;
use crate::util::ApiClient;

pub struct PageState {
    username: UseState<String>,
    password: UseState<String>,
    form_errors: KeyedNotifications,
}

impl PageState {
    pub fn new(cx: Scope) -> Self {
        Self {
            username: use_state(cx, String::new).clone(),
            password: use_state(cx, String::new).clone(),
            form_errors: KeyedNotifications::default(),
        }
    }

    pub fn can_submit(&self) -> bool {
        !(self.form_errors.has_messages()
        || self.username.current().is_empty()
        || self.password.current().is_empty())
    }
}

#[inline_props]
pub fn UsernameInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "username",
                "Username"
            },
            input {
                id: "username",
                name: "username",
                class: "input-field",
                placeholder: "Username",
                value: "{state.current()}",
                oninput: move |ev| oninput.call(ev),
            }
        }
    })
}

#[inline_props]
pub fn PasswordInput<'a>(
    cx: Scope<'a>,
    state: UseState<String>,
    oninput: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            label {
                r#for: "password",
                "Password"
            },
            input {
                class: "input-field",
                r#type: "password",
                id: "password",
                name: "password",
                placeholder: "Password",
                value: "{state.current()}",
                oninput: move |ev| oninput.call(ev),
            }
        }
    })
}

pub fn Register(cx: Scope) -> Element {
    let api_client = ApiClient::global();
    // let username = use_state(cx, String::new);
    // let password = use_state(cx, String::new);

    let page_state = PageState::new(cx);
    let page_state = use_ref(cx, || page_state);

    let router = use_router(&cx);
    let local_profile = use_local_profile(cx);

    let form_onsubmit = async_handler!(&cx, [api_client, page_state, router, local_profile], move |_|
        async move {
            use uchat_endpoint::user::endpoint::{CreateUser, CreateUserOk};
            let request_data = {
                use uchat_domain::{Username, Password};
                CreateUser {
                    username: Username::try_new(page_state.with(
                        |state| state.username.current().to_string())
                    ).unwrap(),
                    password: Password::try_new(page_state.with(
                        |state| state.password.current().to_string())
                    ).unwrap(),
                }
            };
            let response = fetch_json!(<CreateUserOk>, api_client, request_data);
            match response {
                Ok(res) => {
                    crate::util::cookie::set_session(
                        res.session_signature,
                        res.session_id,
                        res.session_expires,
                    );

                    local_profile.write().user_id = Some(res.user_id);
                    router.navigate_to(page::HOME)
                },
                Err(e) => ()
            }
        }
    );

    let username_oninput = sync_handler!(
        [page_state],
        move |ev: FormEvent| {
            if let Err(e) = uchat_domain::Username::try_new(&ev.value) {
                page_state.with_mut(|state| state.form_errors.set("用户名错误", e.formatted_error()));
            } else {
                page_state.with_mut(|state| state.form_errors.remove("用户名错误"));
            };
            page_state.with_mut(|state| state.username.set(ev.value.clone()));
        }
    );

    let password_oninput = sync_handler!(
        [page_state],
        move |ev: FormEvent| {
            if let Err(e) = uchat_domain::Password::try_new(&ev.value) {
                page_state.with_mut(|state| state.form_errors.set("密码错误", e.formatted_error()));
            } else {
                page_state.with_mut(|state| state.form_errors.remove("密码错误"));
            };
            page_state.with_mut(|state| state.password.set(ev.value.clone()));
        }
    );

    let submit_btn_style = maybe_class!("btn-disabled", !page_state.with(|state| state.can_submit()));

    // let submit_btn_style = match page_state.with(|state| state.can_submit()) {
    //     false => "btn-disabled",
    //     true => "",
    // };

    cx.render(rsx! {
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: form_onsubmit,

            UsernameInput {
                state: page_state.with(|state| state.username.clone()),
                oninput: username_oninput
            },

            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput
            }

            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.clone().with(|state| state.form_errors.clone()),
            }

            button {
                class: "btn {submit_btn_style}",
                disabled: !page_state.with(|state| state.can_submit()),
                r#type: "submit",
                "Sign Up"
            }
        }
    })
}