use chrono::{Datelike, Utc};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{path, SsrMode};

use crate::{home, post};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <link rel="stylesheet" id="leptos" href="/pkg/blog.css" />
                <title>Tech Diaries - The Official Rust-DD Developer Blog</title>
                <meta
                    name="description"
                    content="Tech Diaries - The Official Rust-DD Developer Blog"
                />
                <Meta property="og:type" content="article" />
                <Meta property="og:url" content="https://rust-dd.com/" />
                <Meta property="og:image" content="https://static.rust-dd.com/rust-dd.png" />
                <Meta
                    property="og:site_name"
                    content="Tech Diaries - The Official Rust-DD Developer Blog"
                />
                <Meta
                    property="og:title"
                    content="Tech Diaries - The Official Rust-DD Developer Blog"
                />
                <Meta
                    property="og:description"
                    content="Tech Diaries - The Official Rust-DD Developer Blog"
                />
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <div class="overflow-auto h-screen text-white bg-[#1e1e1e]">
            <header class="fixed top-0 right-0 left-0 z-10 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md">
                <div class="container mx-auto max-w-5xl">
                    <a href="/" class="text-3xl font-bold">
                        blog
                    </a>
                </div>
            </header>
            <main class="container flex flex-col gap-8 py-12 px-4 mx-auto mt-16 max-w-5xl md:px-0">
                <Router>
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/") view=home::Component ssr=SsrMode::Async />
                        <Route path=path!("/post/:slug/") view=post::Component ssr=SsrMode::Async />
                    </Routes>
                </Router>
            </main>
            <footer class="fixed right-0 bottom-0 left-0 z-10 py-4 text-center bg-[#1e1e1e]/80 backdrop-blur-md">
                <p class="text-gray-400">
                    Powered by <a href="https://github.com/rust-dd" class="text-[#ffbd2e]">
                        rust-dd
                    </a> {" © "} {Utc::now().year()}
                </p>
            </footer>
        </div>
    }
}
