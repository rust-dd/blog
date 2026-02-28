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
        document::Meta { name: "theme-color", content: "#1e1e1e" }
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

        div { class: "overflow-auto min-h-screen bg-[#1e1e1e] text-white font-mono",
            Router::<Route> {}
        }
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        header::Component {}
        main { class: "container flex flex-col gap-8 px-4 pt-10 pb-20 mx-auto mt-16 md:px-0",
            SuspenseBoundary {
                fallback: |_| rsx! { loader::Inline { message: "Loading page...".to_string() } },
                Outlet::<Route> {}
            }
        }
        footer { class: "fixed right-0 bottom-0 left-0 z-50 py-2 text-center md:py-4 bg-[#1e1e1e]/80 backdrop-blur-md",
            div { class: "flex flex-col gap-1 justify-center items-center",
                p { class: "text-gray-400",
                    "Powered by"
                    a {
                        href: "https://github.com/rust-dd",
                        class: "hover:underline text-[#ffef5c]",
                        " rust-dd"
                    }
                    " © {Utc::now().year()}"
                }
                div { class: "block md:hidden",
                    icons::Component {}
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
            h1 { class: "text-4xl font-bold text-[#ffef5c]", "404" }
            p { class: "mt-4 text-lg text-gray-300", "Page not found: {attempted_path}" }
            Link {
                to: Route::Home {},
                class: "inline-flex mt-8 text-[#ffef5c] hover:underline",
                "Go back home"
            }
        }
    }
}
