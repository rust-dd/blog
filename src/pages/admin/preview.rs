use super::state::EditorState;
use crate::ssr::admin::admin_preview;
use dioxus::prelude::*;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[component]
pub fn Preview(state: Signal<EditorState>) -> Element {
    let mut html = use_signal(String::new);
    let mut error = use_signal(String::new);
    let mut pending = use_signal(|| false);
    let mut generation = use_signal(|| 0u64);
    let markdown = use_memo(move || state().input.body);
    use_effect(move || {
        let body = markdown();
        let version = *generation.peek() + 1;
        generation.set(version);
        pending.set(true);
        spawn(async move {
            let mut delay =
                document::eval("await new Promise(resolve => setTimeout(resolve, 350)); dioxus.send(null);");
            let _ = delay.recv::<serde_json::Value>().await;
            if *generation.peek() != version {
                return;
            }
            let result = admin_preview(body).await;
            if *generation.peek() != version {
                return;
            }
            pending.set(false);
            match result {
                Ok(rendered) => {
                    html.set(rendered);
                    error.set(String::new());
                }
                Err(err) => error.set(super::error_message(&err.into())),
            }
        });
    });
    let current = state();
    let title = escape(&current.input.title);
    let summary = escape(&current.input.summary);
    let image = current
        .input
        .header_image
        .as_ref()
        .map(|url| format!("<img class=\"post-header-image\" src=\"{}\" alt=\"\">", escape(url)))
        .unwrap_or_default();
    let stylesheet = asset!("/assets/tailwind.css");
    let rendered = html();
    let srcdoc = format!("<!doctype html><html><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><base target=\"_blank\"><link rel=\"stylesheet\" href=\"{stylesheet}\"><link rel=\"stylesheet\" href=\"/katex.min.css\"><link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css2?family=Fira+Sans:wght@400;500;600;700&family=Source+Code+Pro:wght@400;500;600&family=Source+Serif+4:opsz,wght@8..60,400..700&display=swap\"></head><body class=\"bg-bg text-fg font-sans\"><main class=\"admin-preview-page\"><h1 class=\"doc-h1\">{title}</h1><p class=\"doc-lead\">{summary}</p>{image}<div class=\"prose post-body\">{rendered}</div></main></body></html>");
    let mut hasher = DefaultHasher::new();
    srcdoc.hash(&mut hasher);
    let frame_key = hasher.finish();
    rsx! {
        section { class: "admin-preview-pane", aria_label: "Post preview",
            div { class: "admin-pane-label", "LIVE PREVIEW", span { aria_live: "polite", if pending() { "Updating…" } } }
            if !error().is_empty() { p { class: "admin-error", role: "alert", "{error}" } }
            // A fresh frame keeps preview updates out of the browser's shared session history.
            for key in [frame_key] {
                iframe { key: "{key}", title: "Live post preview", class: "admin-preview-frame", "sandbox": "allow-popups allow-popups-to-escape-sandbox", srcdoc: "{srcdoc}" }
            }
        }
    }
}
