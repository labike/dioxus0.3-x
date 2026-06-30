#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Values;

use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct KeyedNotifications {
    pub inner: HashMap<String, String>,
}

impl KeyedNotifications {
    pub fn set<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.inner.insert(key.into(), value.into());
    }

    pub fn remove<K: AsRef<str>>(&mut self, key: K) {
        self.inner.remove(key.as_ref());
    }

    pub fn messages(&self) -> Values<'_, String, String> {
        self.inner.values()
    }

    pub fn has_messages(&self) -> bool {
        !self.inner.is_empty()
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct KeyedNotificationsProps {
    legend: Option<String>,
    notifications: KeyedNotifications,
}

#[component]
pub fn KeyedNotificationBox(props: KeyedNotificationsProps) -> Element {
    let notifications = props.notifications.messages().map(|msg| {
        rsx! {
            li { "{msg}" }
        }
    }).collect::<Vec<_>>();

    let legend = props.legend.unwrap_or_else(|| "Errors".to_string());

    match props.notifications.has_messages() {
        true => rsx! {
            fieldset { class: "fieldset border-red-300 rounded",
                legend { class: "bg-red-300 px-4", "{legend}" }
                ul { class: "list-disc ml-4", for notification in notifications { {notification} } }
            }
        },
        false => rsx! { span {} },
    }
}
