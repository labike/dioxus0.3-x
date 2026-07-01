#![allow(non_snake_case)]

use crate::prelude::{app_bar, use_toaster, Appbar, AppbarImgButton};
use crate::util::ApiClient;
use crate::{async_handler, fetch_json, maybe_class, page};
use chrono::Duration;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uchat_domain::ids::PollChoiceId;
use uchat_domain::post::{PollChoiceDescription, PollHeading};
use uchat_endpoint::post::endpoint::{NewPost, NewPostOk};
use uchat_endpoint::post::types::{NewPostOptions, Poll, PollChoice};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageState {
    pub heading: String,
    pub poll_choices: BTreeMap<usize, String>,
    pub next_id: usize,
}

impl Default for PageState {
    fn default() -> Self {
        Self {
            heading: "".to_string(),
            poll_choices: {
                let mut map = BTreeMap::new();
                map.insert(0, "".to_string());
                map.insert(1, "".to_string());
                map
            },
            next_id: 2,
        }
    }
}

impl PageState {
    pub fn can_submit(&self) -> bool {
        if PollHeading::try_new(&self.heading).is_err() {
            return false;
        }

        if self.poll_choices.len() < 2 {
            return false;
        }

        if self
            .poll_choices
            .values()
            .map(PollChoiceDescription::try_new)
            .collect::<Result<Vec<PollChoiceDescription>, _>>()
            .is_err()
        {
            return false;
        }

        true
    }

    pub fn push_choice<T: Into<String>>(&mut self, choice: T) {
        self.poll_choices.insert(self.next_id, choice.into());
        self.next_id += 1;
    }

    pub fn replace_choice<T: Into<String>>(&mut self, key: usize, choice: T) {
        self.poll_choices.insert(key, choice.into());
    }
}

#[component]
pub fn HeadingInput(page_state: Signal<PageState>) -> Element {
    let max_chars = PollHeading::MAX_CHARS;

    let wrong_len = maybe_class!(
        "err-text-color",
        page_state.read().heading.len() > max_chars || page_state.read().heading.is_empty()
    );

    rsx! {
        div {
            label {
                r#for: "heading",
                div {
                    class: "flex flex-row justify-between",
                    span {
                        "Heading"
                    }
                    span {
                        class: "text-right {wrong_len}",
                        "{page_state.read().heading.len()}/{max_chars}",
                    }
                }
            },
            input {
                class: "input-field",
                id: "heading",
                value: "{page_state.read().heading}",
                oninput: move |ev| {
                    page_state.with_mut(|state| state.heading = ev.value().clone())
                }
            }
        }
    }
}

#[component]
pub fn PollChoices(page_state: Signal<PageState>) -> Element {
    let choices = page_state.read().poll_choices.iter().map(|(&key, choice)| {
        let choice = choice.clone();
        let max_chars = PollChoiceDescription::MAX_CHARS;
        let wrong_len = maybe_class!(
            "err-text-color",
           PollChoiceDescription::try_new(&choice).is_err()
        );

        rsx! {
            li {
                key: "{key}",
                div {
                    class: "flex items-center",
                    input {
                        class: "inout-field flex-1 border-1",
                        placeholder: "",
                        oninput: move |ev| {
                            page_state.with_mut(|state| state.replace_choice(key, ev.value()));
                        },
                        value: "{choice}"
                    }
                    div {
                        class: "text-right {wrong_len} mx-3 my-0",
                        "{choice.len()}/{max_chars}"
                    },
                    button {
                        class: "btn py-0 px-2 h-full bg-red-700",
                        r#type: "button",
                        onclick: move |_| {
                            page_state.with_mut(|state| state.poll_choices.remove(&key));
                        },
                        "X"
                    }
                }
            }
        }
    }).collect::<Vec<Element>>();

    rsx! {
        div {
            class: "flex flex-col gap-2",
            "Poll Choices",
            ol {
                class: "list-decimal ml-4 flex flex-col gap-2",
                for choice in choices { {choice} }
            },
            div {
                class: "flex flex-row justify-end",
                button {
                    class: "btn w-12",
                    r#type: "button",
                    onclick: move |_| {
                        page_state.with_mut(|state| state.push_choice(""))
                    },
                    "+"
                }
            }
        }
    }
}
#[component]
pub fn NewPoll() -> Element {
    let api_client = ApiClient::global();
    let navigator = use_navigator();
    let toaster = use_toaster();

    let page_state = use_signal(PageState::default);

    let submit_btn_style = maybe_class!("btn-disabled", !page_state.read().can_submit());

    let form_onsubmit = async_handler!(
        [api_client, page_state, toaster, navigator],
        move |_| async move {
            let request_data = NewPost {
                content: Poll {
                    heading: {
                        let heading = &page_state.read().heading;
                        PollHeading::try_new(heading).unwrap()
                    },
                    choices: {
                        // let stored_choices = {
                        //     let mut choices = page_state.read().poll_choices.iter().map(
                        //         |(id, choice)| (*id, choice.clone())
                        //     ).into_iter().collect::<Vec<(usize, String)>>();
                        //
                        //     choices.sort_unstable_by(
                        //         |a, b| a.0.partial_cmp(&b.0).unwrap()
                        //     );
                        //     choices
                        // };

                        // stored_choices.iter().map(|(_, choice)| {
                        //     let id = PollChoiceId::new();
                        //     let choice = PollChoice {
                        //         id,
                        //         num_votes: 0,
                        //         description: PollChoiceDescription::try_new(choice).unwrap()
                        //     };
                        //     choice
                        // }).collect::<Vec<PollChoice>>()

                        page_state
                            .read()
                            .poll_choices
                            .values()
                            .map(|choice| {
                                let id = PollChoiceId::new();
                                PollChoice {
                                    id,
                                    num_votes: 0,
                                    description: PollChoiceDescription::try_new(choice).unwrap(),
                                }
                            })
                            .collect::<Vec<PollChoice>>()
                    },
                    voted: None,
                }
                .into(),
                options: NewPostOptions::default(),
            };

            let response = fetch_json!(<NewPostOk>, api_client, request_data);

            match response {
                Ok(_) => {
                    toaster
                        .write()
                        .success("Poll Success!", Duration::seconds(3));
                    // 禁止返回post
                    navigator.replace(page::Route::Home {});
                }
                Err(e) => {
                    toaster
                        .write()
                        .error(format!("Poll failed: {e}"), Duration::seconds(3));
                }
            }
        }
    );

    rsx! {
        Appbar {
            title: "Poll",
            AppbarImgButton {
                click_handler: move || {
                    navigator.replace(page::Route::NewChat {});
                },
                img: "/static/icons/icon-messages.svg",
                label: "Chat",
                title: "Post a new chat",
            },
            AppbarImgButton {
                click_handler: move || {
                    navigator.replace(page::Route::NewImage {});
                },
                img: "/static/icons/icon-image.svg",
                label: "Image",
                title: "Post a new image",
            }
            AppbarImgButton {
                click_handler: move || (),
                img: "/static/icons/icon-poll.svg",
                label: "Poll",
                disabled: true,
                title: "Post a new poll",
                append_class: app_bar::BUTTON_SELECTED.to_string(),
            },
            AppbarImgButton {
                click_handler: move || navigator.go_back(),
                img: "/static/icons/icon-back.svg",
                label: "Back",
                title: "Go to the previous page",
            },
        }
        form {
            class: "flex flex-col gap-4",
            onsubmit: move |evt| {
                evt.prevent_default();
                form_onsubmit(evt);
            },
            HeadingInput {
                page_state: page_state.clone(),
            },
            PollChoices {
                page_state: page_state.clone(),
            },
            button {
                class: "btn {submit_btn_style}",
                r#type: "submit",
                disabled: !page_state.read().can_submit(),
                "Poll"
            }
        }
    }
}
