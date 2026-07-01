use crate::prelude::use_toaster;
use crate::util::ApiClient;
use crate::{async_handler, fetch_json, maybe_class};
use chrono::Duration;
use dioxus::prelude::*;
use uchat_domain::post::Message;
use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
use uchat_endpoint::post::types::{Chat, NewPostOptions};

fn can_submit(message: &str) -> bool {
    message.len() <= Message::MAX_CHARS && !message.is_empty()
}

#[component]
pub fn MessageInput(message: String, on_input: EventHandler<FormEvent>) -> Element {
    let max_chars = Message::MAX_CHARS;

    let wrong_len = maybe_class!("err-text-color", !can_submit(&message));

    rsx! {
        div { class: "flex flex-row relative",
            textarea {
                class: "input-field",
                id: "message",
                rows: 3,
                value: "{message}",
                oninput: move |ev| on_input.call(ev),
            }
            div { class: "text-right {wrong_len} absolute bottom-1 right-1",
                "{message.len()}/{max_chars}"
            }
        }
    }
}

#[component]
pub fn QuickRespond(opened: Signal<bool>) -> Element {
    let api_client = ApiClient::global();
    let toaster = use_toaster();

    let mut message = use_signal(String::new);

    let form_onsubmit = async_handler!(
        [api_client, toaster, message, opened],
        move |_| async move {
            let request_data = NewPost {
                content: Chat {
                    heading: None,
                    message: Message::try_new(message.read().as_str()).unwrap(),
                }
                .into(),
                options: NewPostOptions::default(),
            };

            let response = fetch_json!(<NewPostOk>, api_client, request_data);
            match response {
                Ok(_) => {
                    toaster
                        .write()
                        .success("Reply success", Duration::seconds(3));
                    opened.set(false);
                }
                Err(e) => {
                    toaster
                        .write()
                        .error(format!("Reply failed: {e}"), Duration::seconds(3));
                }
            }
        }
    );

    let submit_cursor = if can_submit(&message.read()) {
        "cursor-pointer"
    } else {
        "cursor-not-allowed"
    };

    let submit_btn_style = maybe_class!("btn-disabled", !can_submit(&message.read()));

    rsx! {
        form { onsubmit: move |evt| {
                evt.prevent_default();
                form_onsubmit(evt);
            },
            div { class: "w-full flex flex-col justify-end",
                MessageInput {
                    message: message.read().clone(),
                    on_input: move |ev: FormEvent| { message.set(ev.value().clone()) },
                }
                button {
                    class: "mt-2 btn {submit_cursor} {submit_btn_style} w-[80px] h-[30px] flex justify-center items-center self-end",
                    r#type: "submit",
                    disabled: !can_submit(&message.read()),
                    "Respond"
                }
            }
        }
    }
}
