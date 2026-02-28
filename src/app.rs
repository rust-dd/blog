use chrono::{Datelike, Utc};
use dioxus::prelude::*;

use crate::{
    components::{header, icons, loader},
    pages::{hireus, home, post, references},
    seo,
};

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/post/:slug")]
    Post { slug: String },
    #[route("/references")]
    References {},
    #[route("/hireus")]
    HireUs {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: "/katex.min.css" }
        document::Meta { charset: "utf-8" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta {
            name: "keywords",
            content: "rust-dd, rust, ai, mathematics, embedded, web, systems, programming"
        }
        document::Meta { name: "theme-color", content: "#f8fafc" }
        document::Meta { property: "og:site_name", content: seo::SITE_NAME }
        document::Meta { property: "og:locale", content: "en_US" }
        document::Meta {
            property: "og:image",
            content: seo::DEFAULT_OG_IMAGE
        }
        document::Meta { property: "og:image:type", content: "image/png" }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "627" }
        document::Meta { name: "twitter:site", content: seo::X_HANDLE }
        document::Meta { name: "twitter:creator", content: seo::X_HANDLE }
        document::Meta { name: "twitter:image", content: seo::DEFAULT_OG_IMAGE }
        document::Meta { name: "twitter:image:alt", content: "Rust-DD logo" }
        document::Link {
            rel: "alternate",
            r#type: "application/rss+xml",
            title: "Rust-DD RSS Feed",
            href: seo::absolute_url("/rss.xml")
        }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }

        div { class: "min-h-screen text-slate-900 font-mono",
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
            footer { class: "z-40 border-t border-dashed border-slate-300 py-3",
                div { class: "flex flex-col items-center gap-2",
                    div { class: "block sm:hidden",
                        icons::Component {}
                    }
                    p { class: "text-xs text-slate-400",
                        "// powered by "
                        a {
                            href: "https://github.com/rust-dd",
                            class: "text-slate-500 hover:text-slate-700",
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
    rsx! { post::Component { slug } }
}

#[component]
fn References() -> Element {
    rsx! { references::Component {} }
}

#[component]
fn HireUs() -> Element {
    rsx! { hireus::Component {} }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    let attempted_path = if route.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", route.join("/"))
    };
    let canonical = seo::absolute_url(&attempted_path);

    rsx! {
        document::Title { "404 | Rust-DD" }
        document::Meta { name: "description", content: "This page could not be found on Rust-DD." }
        document::Meta { name: "robots", content: "noindex, nofollow" }
        document::Meta { name: "googlebot", content: "noindex, nofollow" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: "404 | Rust-DD" }
        document::Meta { property: "og:description", content: "This page could not be found on Rust-DD." }
        document::Meta { property: "og:url", content: "{canonical}" }
        document::Meta { name: "twitter:card", content: "summary" }
        document::Meta { name: "twitter:title", content: "404 | Rust-DD" }
        document::Meta { name: "twitter:description", content: "This page could not be found on Rust-DD." }
        document::Meta { name: "twitter:url", content: "{canonical}" }
        document::Link { rel: "canonical", href: "{canonical}" }
        section { class: "mx-auto max-w-3xl text-center pt-24",
            h1 { class: "text-4xl font-bold text-slate-600", "404" }
            p { class: "mt-4 text-lg text-slate-600", "Page not found: {attempted_path}" }
            Link {
                to: Route::Home {},
                class: "inline-flex mt-8 text-slate-600 hover:underline",
                "Go back home"
            }
        }
    }
}
