use dioxus::prelude::*;

use crate::{app::Route, ssr::types::Post};

#[component]
pub fn PostList(posts: Vec<Post>) -> Element {
    rsx! {
        ul { class: "post-list",
            for post in posts {
                li { key: "{post.slug.clone().unwrap_or_default()}", class: "post-row",
                    time { datetime: "{post.created_at}", "{post.published_on()}" }
                    Link {
                        to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                        class: "post-row-title",
                        "{post.title}"
                    }
                    if let Some(topic) = post.topic.clone() {
                        span { class: "post-row-topic", "{topic}" }
                    }
                }
            }
        }
    }
}
