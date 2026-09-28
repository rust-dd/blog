use chrono::{Datelike, Utc};
use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;

use crate::{
    components::{header, icons, loader},
    pages::{home, opensource, post, projects},
    seo,
};

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/post/:slug")]
    Post { slug: String },
    #[route("/projects")]
    Projects {},
    #[route("/opensource")]
    OpenSource {},
    #[end_layout]
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
        document::Meta { name: "theme-color", content: "#fafaf9" }
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
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600;700&display=swap"
        }
        document::Script {
            "try{{var t=localStorage.getItem('theme')||'light';document.documentElement.setAttribute('data-theme',t)}}catch(e){{document.documentElement.setAttribute('data-theme','light')}}"
        }

        div { class: "min-h-screen bg-bg text-fg font-mono",
            Router::<Route> {}
        }
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div { class: "flex min-h-screen flex-col",
            header::Component {}
            main { class: "mx-auto flex w-full max-w-4xl flex-1 flex-col gap-8 px-4 pt-6 pb-20 sm:px-6",
                SuspenseBoundary {
                    fallback: |_| rsx! { loader::Inline { message: "Loading page...".to_string() } },
                    Outlet::<Route> {}
                }
            }
            footer { class: "z-40 border-t border-dashed border-border py-3",
                div { class: "flex flex-col items-center gap-2",
                    div { class: "block sm:hidden",
                        icons::Component {}
                    }
                    p { class: "text-xs text-faint",
                        "// powered by "
                        a {
                            href: "https://github.com/rust-dd",
                            class: "text-muted transition-colors duration-200 hover:text-accent",
                            "rust-dd"
                        }
                        " | {Utc::now().year()}"
                    }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! { home::Component {} }
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
        section { class: "mx-auto max-w-3xl text-center pt-24",
            p { class: "text-xs text-faint", "// route not found" }
            h1 { class: "mt-2 text-5xl font-bold text-accent", "404" }
            p { class: "mt-4 text-lg text-muted", "Page not found: {attempted_path}" }
            Link {
                to: Route::Home {},
                class: "inline-flex mt-8 text-accent hover:underline",
                "Go back home"
            }
        }
    }
}
