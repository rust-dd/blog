use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;

use crate::{
    components::{loader, shell},
    pages::{about, home, opensource, post, projects},
    seo,
};

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(Layout)]
    #[route("/?:q")]
    Home { q: String },
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

        div { class: "min-h-screen bg-bg text-fg font-sans",
            Router::<Route> {}
        }
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        a { href: "#main", class: "skip-link", "Skip to content" }
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
fn Home(q: String) -> Element {
    rsx! { home::Component { query: q } }
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
        main { id: "main", class: "shell-main",
            div { class: "doc-title", style: "margin-top: 36px",
                h1 { class: "doc-h1", "Page not found" }
            }
            p { class: "doc-lead",
                "Nothing lives at "
                code { class: "ident text-mod", "{attempted_path}" }
                "."
            }
            p { class: "doc-text", style: "margin-top: 18px",
                Link { to: Route::Home { q: String::new() }, class: "text-mod", "Back to all posts" }
            }
        }
    }
}
