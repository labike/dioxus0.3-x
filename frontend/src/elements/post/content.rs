#![allow(non_snake_case)]

use crate::prelude::*;
use crate::util::ApiClient;
use crate::{fetch_json, maybe_class};
use dioxus::prelude::*;
use itertools::Itertools;
use std::collections::HashSet;
use uchat_domain::ids::{PollChoiceId, PostId};
use uchat_endpoint::post::endpoint::{Vote, VoteOk};
use uchat_endpoint::post::types::{
    Chat as EndpointChat, Image as EndpointImage, ImageKind, Poll as EndpointPoll, PublicPost,
    VoteCast,
};

#[inline_props]
pub fn Chat(content: &EndpointChat) -> Element {
    let Heading = content.heading.as_ref().map(|heading| {
        rsx! {
            div {
                class: "font-bold",
                "{heading.as_ref()}"
            }
        }
    });

    rsx! {
        div {
            Heading,
            p {
                "{content.message.as_ref()}"
            }
        }
    }
}

#[inline_props]
pub fn Image(content: &EndpointImage) -> Element {
    let url = if let ImageKind::Url(url) = &content.kind {
        url
    } else {
        return cx.render(rsx! {
            "image not found"
        });
    };

    let Caption = content.caption.as_ref().map(|caption| {
        rsx! {
            figcaption {
                em {
                    "{caption.as_ref()}"
                }
            }
        }
    });

    rsx! {
        figure {
            class: "flex flex-col gap2",
            Caption,
            img {
                class: "w-full object-contain max-h-[80vh]",
                src: "{url}"
            }
        }
    }
}

#[inline_props]
pub fn Poll(post_id: PostId, content: &EndpointPoll) -> Element {
    let toaster = use_toaster();
    let api_client = ApiClient::global();

    let vote_onclick = async_handler!(
        [api_client, toaster],
        move |post_id, choice_id| async move {
            let request_data = Vote { post_id, choice_id };

            match fetch_json!(<VoteOk>, api_client, request_data) {
                Ok(res) => match res.cast {
                    VoteCast::Yes => toaster
                        .write()
                        .success("Vote Cast", chrono::Duration::seconds(3)),
                    VoteCast::AlreadyVoted => toaster
                        .write()
                        .info("Already Voted", chrono::Duration::seconds(3)),
                },
                Err(e) => toaster.write().error(
                    format!("Failed to cast vote: {}", e),
                    chrono::Duration::seconds(3),
                ),
            }
        }
    );

    let total_votes = content
        .choices
        .iter()
        .map(|choice| choice.num_votes)
        .sum::<i64>();
    let leader_ids = {
        let leaders = content
            .choices
            .iter()
            .max_set_by(|x, y| x.num_votes.cmp(&y.num_votes));
        let ids: HashSet<PollChoiceId> = HashSet::from_iter(leaders.iter().map(|choice| choice.id));
        ids
    };

    let Choices = content.choices.iter().map(|choice| {
        let percent = if total_votes > 0 {
            let percent = (choice.num_votes as f64 / total_votes as f64) * 100.0;
            format!("{percent:.0}%")
        } else {
            "0%".to_string()
        };

        let background_color = if leader_ids.contains(&choice.id) {
            "bg-blue-300"
        } else {
            "bg-neutral-300"
        };

        let foreground_styles = maybe_class!("font-bold", leader_ids.contains(&choice.id));

        rsx! {
            li {
                key: "{choice.id.to_string()}",
                class: "relative p-2 m-3 cursor-pointer grid grid-cols-3rem_1fr] border rounded border-slate-400",
                onclick: move |_| vote_onclick(*post_id, choice.id),
                div {
                    class: "absolute left-0 {background_color} h-full rounded z-[-1]",
                    style: "width: {percent}",
                },
                div {
                    class: "{foreground_styles}",
                    "{percent}",
                },
                div {
                    class: "{foreground_styles}",
                    "{choice.description.as_ref()}",
                }
            }
        }
    });

    let Heading = rsx! {
        figcaption {
            "{content.heading.as_ref()}"
        }
    };

    rsx! {
        div {
            Heading,
            ul {
                Choices.into_iter()
            }
        }
    }
}

#[inline_props]
pub fn Content(cx: Scope, post: &PublicPost) -> Element {
    use uchat_endpoint::post::types::Content as EndpointContent;
    rsx! {
        div {
            match &post.content {
                EndpointContent::Chat(content) => rsx! {
                    Chat {
                        content: content,
                    }
                },
                EndpointContent::Image(content) => rsx! {
                    Image {
                        content: content,
                    }
                },
                EndpointContent::Poll(content) => rsx! {
                    Poll {
                        post_id: post.id,
                        content: content,
                    }
                },
            }
        }
    }
}
