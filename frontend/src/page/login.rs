#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::Link;
use uchat_domain::UserFacingError;
use crate::elements::keyed_notification_box::{KeyedNotificationBox, KeyedNotifications};
use crate::{fetch_json, maybe_class, page};
use crate::prelude::*;
use crate::util::ApiClient;

pub struct PageState {
    username: Signal<String>,
    password: Signal<String>,
    form_errors: KeyedNotifications,
    server_messages: KeyedNotifications,
}

impl PageState {
    pub fn new() -> Self {
        Self {
            username: use_signal(|| String::new()).clone(),
            password: use_signal(|| String::new()).clone(),
            form_errors: KeyedNotifications::default(),
            server_messages: KeyedNotifications::default(),
        }
    }

    pub fn can_submit(&self) -> bool {
        !(self.form_errors.has_messages()
            || self.username.is_empty()
            || self.password.is_empty())
    }
}

#[component]
pub fn UsernameInput(
    state: Signal<String>,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col",
            label { r#for: "username", "Username" }
            input {
                id: "username",
                name: "username",
                class: "input-field",
                placeholder: "Username",
                value: "{state.read()}",
                oninput: move |ev| oninput.call(ev),
            }
        }
    }
}

#[component]
pub fn PasswordInput(
    state: Signal<String>,
    oninput: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col",
            label { r#for: "password", "Password" }
            input {
                class: "input-field",
                r#type: "password",
                id: "password",
                name: "password",
                placeholder: "Password",
                value: "{state.read()}",
                oninput: move |ev| oninput.call(ev),
            }
        }
    }
}


#[component]
pub fn RegisterLink() -> Element {
    rsx! {
        Link { class: "link text-center", to: page::Route::Register {}, "Create Account" }
    }
}

#[component]
pub fn Login() -> Element {
    let api_client = ApiClient::global();
    let page_state = use_signal(PageState::new);
    let navigator = use_navigator();
    let local_profile = use_local_profile();

    let form_onsubmit = async_handler!([api_client, page_state, navigator, local_profile], move |_|
        async move {
            use uchat_endpoint::user::endpoint::{Login, LoginOk};

            let request_data = {
                use uchat_domain::{Username, Password};

                Login {
                    username: Username::try_new(
                        page_state.with(
                            |state| state.username.read().to_string()
                        )
                    ).unwrap(),
                    password: Password::try_new(
                        page_state.with(
                            |state| state.password.read().to_string()
                        )
                    ).unwrap(),
                }
            };

            let response = fetch_json!(<LoginOk>, api_client, request_data);
            match response {
                Ok(res) => {
                    crate::util::cookie::set_session(
                        res.session_signature,
                        res.session_id,
                        res.session_expires,
                    );

                    local_profile.write().image = res.profile_image;
                    local_profile.write().user_id = Some(res.user_id);
                    navigator.push(page::Route::Home {});
                },
                Err(e) => {
                    page_state.with_mut(|state| state.server_messages.set("login-fail", e.to_string()))
                }
            }
        }
    );

    let username_oninput = sync_handler!(
        [page_state],
        move |ev: FormEvent| {
            if let Err(e) = uchat_domain::Username::try_new(&ev.value()) {
                page_state.with_mut(|state| state.form_errors.set("用户名错误", e.formatted_error()));
            } else {
                page_state.with_mut(|state| state.form_errors.remove("用户名错误"));
            };
            page_state.with_mut(|state| state.username.set(ev.value().clone()));
        }
    );

    let password_oninput = sync_handler!(
        [page_state],
        move |ev: FormEvent| {
            if let Err(e) = uchat_domain::Password::try_new(&ev.value()) {
                page_state.with_mut(|state| state.form_errors.set("密码错误", e.formatted_error()));
            } else {
                page_state.with_mut(|state| state.form_errors.remove("密码错误"));
            };
            page_state.with_mut(|state| state.password.set(ev.value().clone()));
        }
    );

    let submit_btn_style = maybe_class!("btn-disabled", !page_state.with(|state| state.can_submit()));

    // let submit_btn_style = match page_state.with(|state| state.can_submit()) {
    //     false => "btn-disabled",
    //     true => "",
    // };

    rsx! {
        form {
            class: "flex flex-col gap-5",
            prevent_default: "onsubmit",
            onsubmit: form_onsubmit,

            KeyedNotificationBox {
                legend: "Login Errors",
                notifications: page_state.clone().with(|state| state.server_messages.clone()),
            }

            UsernameInput {
                state: page_state.with(|state| state.username.clone()),
                oninput: username_oninput,
            }

            PasswordInput {
                state: page_state.with(|state| state.password.clone()),
                oninput: password_oninput,
            }

            RegisterLink {}

            KeyedNotificationBox {
                legend: "Form Errors",
                notifications: page_state.clone().with(|state| state.form_errors.clone()),
            }

            button {
                class: "btn {submit_btn_style}",
                disabled: !page_state.with(|state| state.can_submit()),
                r#type: "submit",
                "Sign In"
            }
        }
    }
}
