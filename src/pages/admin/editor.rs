use super::{
    metadata::Metadata,
    preview::Preview,
    state::{perform, remove, Action, EditorState},
};
use crate::{
    app::Route,
    ssr::admin::{admin_authors, admin_post, types::PostInput},
};
use dioxus::prelude::*;
use dioxus::CapturedError;

#[component]
pub fn Editor(id: Option<String>) -> Element {
    let data = use_server_future(move || {
        let id = id.clone();
        async move {
            let authors = admin_authors().await?;
            let post = match id {
                Some(id) => Some(admin_post(id).await?),
                None => None,
            };
            Ok::<_, CapturedError>((authors, post))
        }
    })?;
    let Some(result) = data() else {
        return rsx! { "Loading editor…" };
    };
    let (authors, post) = match result {
        Ok(data) => data,
        Err(error) => return rsx! { p { class: "admin-error", role: "alert", "{super::error_message(&error)}" } },
    };
    let mut state = use_signal(|| {
        let input = post
            .as_ref()
            .map(|post| post.input.clone())
            .unwrap_or_else(|| PostInput {
                author_id: authors
                    .iter()
                    .find(|author| author.name == "Daniel Boros")
                    .or(authors.first())
                    .map(|author| author.id.clone())
                    .unwrap_or_default(),
                ..PostInput::default()
            });
        EditorState {
            post,
            saved: input.clone(),
            input,
            busy: false,
            error: String::new(),
            notice: String::new(),
            confirm_delete: false,
        }
    });
    let mut tab = use_signal(|| "split".to_string());
    use_effect(move || {
        let dirty = state().dirty();
        let _ = document::eval(&format!("window.rdAdminDirty = {dirty};"));
    });
    use_drop(|| {
        let _ = document::eval("window.rdAdminDirty = false;");
    });
    let current = state();
    let status = if current.published() { "Published" } else { "Draft" };
    let heading = if current.post.is_some() {
        "Edit post"
    } else {
        "New post"
    };

    rsx! {
        main { class: "admin-editor",
            div { class: "admin-editor-top",
                Link { to: Route::Admin {}, class: "admin-link", "← All posts" }
                div { class: "admin-save-status", aria_live: "polite",
                    if current.busy { "Saving…" } else if current.dirty() { "Unsaved changes" } else if current.post.is_none() { "New draft" } else { "All changes saved" }
                }
            }
            div { class: "admin-page-heading",
                div { div { class: "admin-eyebrow", "WRITING DESK" } h1 { "{heading}" } }
                div { class: "admin-editor-actions",
                    span { class: if current.published() { "admin-badge published" } else { "admin-badge" }, "{status}" }
                    button { id: "admin-save", class: "admin-button", disabled: current.busy,
                        onclick: move |_| async move { perform(state, Action::Save).await; }, "Save" }
                    if current.published() {
                        button { class: "admin-button", disabled: current.busy,
                            onclick: move |_| async move { perform(state, Action::Unpublish).await; }, "Unpublish" }
                    } else {
                        button { class: "button-primary", disabled: current.busy,
                            onclick: move |_| async move { perform(state, Action::Publish).await; }, "Save & publish →" }
                    }
                }
            }
            if !current.error.is_empty() { p { class: "admin-error", role: "alert", "{current.error}" } }
            if !current.notice.is_empty() { p { class: "admin-notice", role: "status", "{current.notice}" } }
            fieldset { disabled: current.busy, class: "admin-fields",
                label { r#for: "post-title", "Title" }
                input { id: "post-title", class: "admin-title-input", placeholder: "What are you writing about?", value: "{current.input.title}", maxlength: "300",
                    oninput: move |event| state.write().input.title = event.value() }
                label { r#for: "post-summary", "Summary" }
                textarea { id: "post-summary", rows: "2", placeholder: "A short introduction for the post list and search results.", maxlength: "2000", value: "{current.input.summary}",
                    oninput: move |event| state.write().input.summary = event.value() }
                Metadata { state, authors }
            }
            div { class: "admin-editor-toolbar",
                div { class: "admin-filter-tabs", aria_label: "Editor view",
                    for (value, label) in [("write", "Write"), ("split", "Split"), ("preview", "Preview")] {
                        button { class: if tab() == value { "admin-filter active" } else { "admin-filter" }, aria_pressed: tab() == value,
                            onclick: move |_| tab.set(value.to_string()), "{label}" }
                    }
                }
                span { class: "admin-muted", "Markdown · Ctrl/⌘ S to save" }
            }
            div { class: "admin-writing-area", "data-view": "{tab}",
                div { class: "admin-write-pane",
                    label { r#for: "post-body", class: "admin-pane-label", "MARKDOWN" }
                    textarea { id: "post-body", class: "admin-markdown-input", spellcheck: "false", disabled: current.busy,
                        placeholder: "## A new idea\n\nStart writing here…", value: "{current.input.body}",
                        oninput: move |event| state.write().input.body = event.value() }
                }
                Preview { state }
            }
            div { class: "admin-editor-bottom",
                p { class: "admin-muted", "Use Markdown image links for hosted images. Saving keeps the current publication status." }
                if let Some(post) = &current.post {
                    div { class: "admin-bottom-actions",
                        if current.published() {
                            if let Some(slug) = &post.slug {
                                a { href: "/post/{slug}", target: "_blank", rel: "noopener noreferrer", class: "admin-link", "View live post ↗" }
                            }
                        }
                        button { class: "admin-danger-link", disabled: current.busy, onclick: move |_| state.write().confirm_delete = true, "Delete post" }
                    }
                }
            }
            if current.confirm_delete {
                div { class: "admin-modal-backdrop",
                    div { class: "admin-modal", role: "alertdialog", aria_modal: "true", aria_labelledby: "delete-title", aria_describedby: "delete-description",
                        h2 { id: "delete-title", "Delete this post?" }
                        p { id: "delete-description", "“{current.input.title}” will be permanently deleted. This cannot be undone." }
                        div { class: "admin-modal-actions",
                            button { class: "admin-button", disabled: current.busy, onclick: move |_| state.write().confirm_delete = false, "Keep post" }
                            button { class: "admin-danger-button", disabled: current.busy, onclick: move |_| async move { remove(state).await; }, "Delete permanently" }
                        }
                    }
                }
            }
        }
    }
}
