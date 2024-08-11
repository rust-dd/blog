use crate::error_template::{AppError, ErrorTemplate};
use crate::{home, post};
use chrono::{Datelike, Utc};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/blog.css" />
        <Title text="Tech Diaries - The Official Rust-DD Developer Blog" />
        <div class="overflow-auto h-screen text-white bg-[#1e1e1e]">
            <header class="fixed top-0 right-0 left-0 z-10 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md">
                <div class="container mx-auto max-w-5xl">
                    <a href="/" class="text-3xl font-bold">
                        blog
                    </a>
                </div>
            </header>
            <main class="container flex overflow-auto flex-col gap-8 py-12 px-4 mx-auto mt-16 max-w-5xl md:px-0">
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors /> }.into_view()
                }>
                    <Routes>
                        <Route path="" view=home::Component ssr=SsrMode::Async />
                        <Route path="/post/:slug" view=post::Component ssr=SsrMode::Async />
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
