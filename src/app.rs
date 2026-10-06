use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;

use crate::{
    components::{
        loader,
        shell::{self, CONTENT_ID},
    },
    pages::{about, admin, home, opensource, post, projects},
    search::SearchQuery,
    seo,
};

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(AdminLayout)]
    #[route("/admin")]
    Admin {},
    #[route("/admin/new")]
    AdminNew {},
    #[route("/admin/edit/:id")]
    AdminEdit { id: String },
    #[end_layout]
    #[layout(Layout)]
    #[route("/?:..query")]
    Home { query: SearchQuery },
    #[route("/post/:slug")]
    Post { slug: String },
    #[route("/about")]
    About {},
    #[route("/projects")]
    Projects {},
    #[route("/opensource")]
    OpenSource {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        // Fallback only; every page sets its own title, and the last one set wins.
        document::Title { "{seo::SITE_NAME}" }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: "/katex.min.css" }
        document::Link { rel: "icon", href: "/favicon.ico" }
        document::Meta { name: "theme-color", content: "#1A1210" }
        document::Meta { property: "og:site_name", content: seo::SITE_NAME }
        document::Meta { property: "og:locale", content: "en_US" }
        document::Meta { name: "twitter:site", content: seo::X_HANDLE }
        document::Meta { name: "twitter:creator", content: seo::X_HANDLE }
        document::Link {
            rel: "alternate",
            r#type: "application/rss+xml",
            title: "Rust-DD RSS Feed",
            href: seo::absolute_url("/rss.xml")
        }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Fira+Sans:ital,wght@0,400;0,500;0,600;0,700;1,400&family=Source+Code+Pro:wght@400;500;600&family=Source+Serif+4:ital,opsz,wght@0,8..60,400..700;1,8..60,400..700&display=swap"
        }
        document::Script {
            "document.addEventListener('keydown',function(e){{var t=e.target;if(e.metaKey||e.ctrlKey||e.altKey||(t&&(t.tagName==='INPUT'||t.tagName==='TEXTAREA'||t.isContentEditable)))return;if(e.key==='s'||e.key==='S'||e.key==='/'){{var i=document.getElementById('search');if(i){{e.preventDefault();i.focus();}}}}}});"
        }
        document::Script {
            src: asset!("/assets/admin-navigation.js")
        }

        div { class: "min-h-screen bg-bg text-fg font-sans",
            Router::<Route> {}
        }
        dialog { id: "admin-unsaved-dialog", class: "admin-confirm-dialog",
            aria_labelledby: "admin-unsaved-title", aria_describedby: "admin-unsaved-description",
            div { class: "admin-eyebrow", "BEFORE YOU GO" }
            h2 { id: "admin-unsaved-title", "Keep your latest words?" }
            p { id: "admin-unsaved-description", "You have changes that haven’t been saved. Stay here to keep writing, or leave without saving them." }
            div { class: "admin-modal-actions",
                button { id: "admin-unsaved-stay", class: "admin-button", autofocus: true, "Stay here" }
                button { id: "admin-unsaved-leave", class: "admin-danger-button", "Leave without saving" }
            }
        }
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        a { href: "#{CONTENT_ID}", class: "skip-link", "Skip to content" }
        div { class: "shell",
            div { class: "shell-search", shell::SearchBar {} }
            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "loader-wrap", loader::Inline { message: "Loading page...".to_string() } }
                },
                Outlet::<Route> {}
            }
            shell::Footer {}
        }
    }
}

#[component]
fn AdminLayout() -> Element {
    rsx! {
        document::Meta { name: "robots", content: "noindex, nofollow" }
        document::Title { "Admin · Rust-DD" }
        SuspenseBoundary {
            fallback: |_| rsx! { div { class: "admin-loading", "Opening your writing desk…" } },
            admin::Gate {}
        }
    }
}

#[component]
fn Admin() -> Element {
    rsx! { admin::List {} }
}

#[component]
fn AdminNew() -> Element {
    rsx! { admin::editor::Editor { id: None } }
}

#[component]
fn AdminEdit(id: String) -> Element {
    rsx! {
        for id in [id] {
            admin::editor::Editor { key: "{id}", id: Some(id) }
        }
    }
}

#[component]
fn Home(query: SearchQuery) -> Element {
    rsx! { home::Component { query: query.0 } }
}

#[component]
fn Post(slug: String) -> Element {
    // Keys only apply inside a list, hence the one-item loop: moving between posts has to remount
    // the page, since it reads its data and head tags once per mount.
    rsx! {
        for slug in [slug] {
            post::Component { key: "{slug}", slug }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! { about::Component {} }
}

#[component]
fn Projects() -> Element {
    rsx! { projects::Component {} }
}

#[component]
fn OpenSource() -> Element {
    rsx! { opensource::Component {} }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    let attempted_path = if route.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", route.join("/"))
    };
    FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);

    rsx! {
        seo::PageMeta {
            title: seo::page_title("Page not found"),
            description: "This page could not be found on Rust-DD.",
            path: attempted_path.clone(),
            noindex: true,
        }
        shell::Sidebar {}
        main { id: CONTENT_ID, tabindex: "-1", class: "shell-main",
            div { class: "doc-title", style: "margin-top: 36px",
                h1 { class: "doc-h1", "Page not found" }
            }
            p { class: "doc-lead",
                "Nothing lives at "
                code { class: "ident text-mod", "{attempted_path}" }
                "."
            }
            p { class: "doc-text", style: "margin-top: 18px",
                Link { to: Route::Home { query: SearchQuery::default() }, class: "text-mod", "Back to all posts" }
            }
        }
    }
}
