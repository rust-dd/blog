pub mod editor;
mod metadata;
pub mod preview;
mod state;

use crate::{
    app::Route,
    search::SearchQuery,
    ssr::admin::{admin_login, admin_logout, admin_posts, admin_session},
};
use dioxus::prelude::*;
use dioxus::CapturedError;

pub fn error_message(error: &CapturedError) -> String {
    match error.downcast_ref::<dioxus::fullstack::ServerFnError>() {
        Some(dioxus::fullstack::ServerFnError::ServerError { message, .. }) => message.clone(),
        _ => "The request failed. Check your connection and try again.".into(),
    }
}

#[component]
pub fn Gate() -> Element {
    let session = use_server_future(admin_session)?;
    let initial = session().and_then(|result| result.ok());
    let mut authenticated = use_signal(|| initial.as_ref().is_some_and(|session| session.authenticated));
    let configured = initial.as_ref().is_some_and(|session| session.configured);
    let mut logout_error = use_signal(String::new);
    use_context_provider(|| authenticated);

    rsx! {
        div { class: "admin-shell",
            header { class: "admin-header",
                Link { to: Route::Admin {}, class: "admin-brand", "rust_dd", span { " / writing desk" } }
                nav { aria_label: "Admin navigation",
                    Link { to: Route::Home { query: SearchQuery::default() }, class: "admin-link", "View site ↗" }
                    if authenticated() {
                        button { class: "admin-link", onclick: move |_| async move {
                            let mut confirmation = document::eval("dioxus.send(!window.rdAdminDirty || await window.rdAdminConfirm('signout'));");
                            if !confirmation.recv::<bool>().await.unwrap_or(false) { return; }
                            match admin_logout().await {
                                Ok(()) => { authenticated.set(false); logout_error.set(String::new()); },
                                Err(error) => logout_error.set(error_message(&error.into())),
                            }
                        }, "Sign out" }
                    }
                }
            }
            if !logout_error().is_empty() { p { class: "admin-error", role: "alert", "{logout_error}" } }
            if authenticated() {
                SuspenseBoundary {
                    fallback: |_| rsx! { div { class: "admin-loading", "Loading…" } },
                    Outlet::<Route> {}
                }
            } else {
                Login { configured }
            }
        }
    }
}

#[component]
fn Login(configured: bool) -> Element {
    let mut password = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut authenticated = use_context::<Signal<bool>>();
    rsx! {
        main { class: "admin-login",
            div { class: "admin-eyebrow", "RUST-DD / ADMIN" }
            h1 { "A place to write." }
            p { class: "admin-muted", "Sign in to turn your ideas into the next post." }
            if !configured {
                p { class: "admin-error", role: "alert", "Admin sign-in is not set up yet. Configure access on the server to start writing." }
            }
            form { onsubmit: move |event| async move {
                event.prevent_default();
                if busy() || !configured { return; }
                busy.set(true);
                error.set(String::new());
                let result = admin_login(password()).await;
                password.set(String::new());
                busy.set(false);
                match result { Ok(()) => authenticated.set(true), Err(err) => error.set(error_message(&err.into())) }
            },
                label { r#for: "admin-password", "Password" }
                input { id: "admin-password", r#type: "password", autocomplete: "current-password", required: true,
                    value: "{password}", disabled: busy() || !configured,
                    oninput: move |event| password.set(event.value()) }
                if !error().is_empty() { p { class: "admin-error", role: "alert", "{error}" } }
                button { r#type: "submit", class: "button-primary", disabled: busy() || !configured,
                    if busy() { "Signing in…" } else { "Sign in →" }
                }
            }
            p { class: "admin-login-note admin-muted", "Your drafts stay private until you publish them." }
        }
    }
}

#[component]
pub fn List() -> Element {
    let posts = use_server_future(admin_posts)?;
    let Some(result) = posts() else {
        return rsx! { "Loading posts…" };
    };
    let posts = match result {
        Ok(posts) => posts,
        Err(error) => {
            let message = error_message(&error.into());
            return rsx! { p { class: "admin-error", role: "alert", "{message}" } };
        }
    };
    let mut search = use_signal(String::new);
    let mut status = use_signal(|| "all".to_string());
    let published = posts.iter().filter(|post| post.is_published).count();
    let drafts = posts.len() - published;
    let query = search().trim().to_lowercase();
    let filtered = posts
        .iter()
        .filter(|post| {
            (query.is_empty() || post.input.title.to_lowercase().contains(&query))
                && match status().as_str() {
                    "draft" => !post.is_published,
                    "published" => post.is_published,
                    _ => true,
                }
        })
        .collect::<Vec<_>>();

    rsx! {
        main { class: "admin-list",
            div { class: "admin-page-heading",
                div { div { class: "admin-eyebrow", "YOUR WORDS, IN ONE PLACE" } h1 { "Posts" }
                    p { class: "admin-muted", "{published} published · {drafts} drafts" }
                }
                Link { to: Route::AdminNew {}, class: "button-primary", "+ New post" }
            }
            div { class: "admin-filters",
                input { r#type: "search", aria_label: "Search posts", placeholder: "Find a post…", value: "{search}",
                    oninput: move |event| search.set(event.value()) }
                div { class: "admin-filter-tabs", aria_label: "Filter posts",
                    for (value, label) in [("all", "All"), ("draft", "Drafts"), ("published", "Published")] {
                        button { class: if status() == value { "admin-filter active" } else { "admin-filter" },
                            aria_pressed: status() == value,
                            onclick: move |_| status.set(value.to_string()), "{label}" }
                    }
                }
            }
            div { class: "admin-posts",
                if filtered.is_empty() {
                    div { class: "admin-empty", h2 { "Room for a new idea." } p { "No posts match this view." } }
                }
                for post in filtered {
                    Link { key: "{post.id}", to: Route::AdminEdit { id: post.id.clone() }, class: "admin-post-row",
                        div { class: "admin-post-copy", h2 { "{post.input.title}" }
                            p { class: "admin-muted", "{post.input.summary}" }
                            div { class: "admin-post-meta",
                                if let Some(topic) = &post.input.topic { span { class: "ident", "{topic}" } }
                                span { "{post.created_at.get(..10).unwrap_or(&post.created_at)}" }
                            }
                        }
                        div { class: "admin-row-end",
                            span { class: if post.is_published { "admin-badge published" } else { "admin-badge" },
                                if post.is_published { "Published" } else { "Draft" }
                            }
                            span { aria_hidden: "true", "→" }
                        }
                    }
                }
            }
        }
    }
}
