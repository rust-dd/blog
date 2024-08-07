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
    // let navigate = use_navigate();

    view! {
        <Stylesheet id="leptos" href="/pkg/blog.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />
        <div class="overflow-auto h-screen text-white bg-[#1e1e1e]">
            <header class="fixed top-0 right-0 left-0 z-10 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md">
                // on:click=move |_| navigate("/", Default::default())
                <div class="container mx-auto max-w-5xl">
                    <h1 class="text-3xl font-bold">blog</h1>
                </div>
            </header>
            <main class="container flex overflow-auto flex-col gap-8 py-12 px-4 mx-auto mt-24 max-w-5xl md:px-0">
                // content for this welcome page
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors /> }.into_view()
                }>
                    <Routes>
                        <Route path="" view=home::Component ssr=SsrMode::Async />
                        <Route path="/post/:id" view=post::Component ssr=SsrMode::Async />
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
