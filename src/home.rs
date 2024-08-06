use chrono::{Datelike, Utc};
use leptos::*;
use leptos_router::use_navigate;

/// Renders the home page of your application.
#[component]
pub fn Component() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <div class="overflow-auto h-screen text-white bg-[#1e1e1e]">
            <header class="fixed top-0 right-0 left-0 z-10 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md">
                <div class="container mx-auto max-w-5xl">
                    <h1 class="text-3xl font-bold">blog</h1>
                </div>
            </header>
            <main class="container flex overflow-auto flex-col gap-8 py-12 px-4 mx-auto mt-24 max-w-5xl md:px-0">
                <For
                    each=move || 0..15
                    key=|idx| idx.to_string()
                    children=move |idx| {
                        let navigate = navigate.clone();
                        view! {
                            <article
                                on:click=move |_| navigate(
                                    &format!("/post/{}", idx),
                                    Default::default(),
                                )
                                class="p-6 rounded-lg shadow-sm transition-transform duration-300 cursor-pointer hover:shadow-lg hover:-translate-y-2"
                            >
                                <div class="flex justify-between items-center mb-4">
                                    <h2 class="text-xl font-semibold">
                                        Mastering the Art of Minimalism
                                    </h2>
                                    <div class="text-sm text-muted-foreground">1.2K</div>
                                </div>
                                <p class="text-muted-foreground">
                                    Discover the power of minimalism and how it can transform your life.
                                </p>
                            </article>
                        }
                    }
                />
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
