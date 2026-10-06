use super::state::EditorState;
use crate::{ssr::admin::types::AdminAuthor, topics::TOPICS};
use dioxus::prelude::*;

#[component]
pub fn Metadata(mut state: Signal<EditorState>, authors: Vec<AdminAuthor>) -> Element {
    let input = state().input;
    let mut tags = use_signal(|| input.tags.join(", "));
    let topic = input.topic.unwrap_or_default();
    let image = input.header_image.unwrap_or_default();
    rsx! {
        div { class: "admin-metadata",
            div { label { r#for: "post-author", "Author" }
                select { id: "post-author", value: "{input.author_id}", oninput: move |event| state.write().input.author_id = event.value(),
                    if authors.is_empty() { option { value: "", "No authors available" } }
                    for author in authors { option { value: "{author.id}", selected: input.author_id == author.id, "{author.name}" } }
                }
            }
            div { label { r#for: "post-topic", "Topic" }
                select { id: "post-topic", value: "{topic}", oninput: move |event| {
                    let value = event.value();
                    state.write().input.topic = (!value.is_empty()).then_some(value);
                },
                    option { value: "", selected: topic.is_empty(), "No topic" }
                    for item in TOPICS { option { value: "{item.name}", selected: topic == item.name, "{item.name}" } }
                }
            }
            div { label { r#for: "post-tags", "Tags" }
                input { id: "post-tags", placeholder: "rust, async, systems", value: "{tags}",
                    oninput: move |event| {
                        tags.set(event.value());
                        state.write().input.tags = event.value().split(',').map(str::trim).filter(|tag| !tag.is_empty()).map(str::to_string).collect();
                    } }
            }
        }
        details { class: "admin-options",
            summary { "More options" }
            div { class: "admin-extra-fields",
                div { label { r#for: "post-image", "Header image URL" }
                    input { id: "post-image", r#type: "url", placeholder: "https://…", value: "{image}", oninput: move |event| {
                        let value = event.value();
                        state.write().input.header_image = (!value.trim().is_empty()).then(|| value.trim().to_string());
                    } }
                }
                label { class: "admin-checkbox", input { r#type: "checkbox", checked: input.show_cta,
                    onchange: move |event| state.write().input.show_cta = event.checked() }, "Show contact invitation at the end of this post" }
            }
        }
    }
}
