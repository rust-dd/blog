use crate::{
    components::{error_template, header},
    pages::{hireus, home, post, references},
};
use chrono::{Datelike, Utc};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    ParamSegment, SsrMode, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html> 
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <Stylesheet id="leptos" href="/pkg/blog.css" />
                <Stylesheet id="katex" href="/katex.min.css" />
                <Title text="Tech Diaries - The Official Rust-DD Developer Blog" />
                <Meta name="hostname" content="rust-dd.com" />
                <Meta name="expected-hostname" content="rust-dd.com" />
                <Meta
                    name="description"
                    content="Discover the Rust-DD framework, enabling the application of domain-driven design (DDD) principles in Rust. Write efficient, safe, and clean code with this modern development tool."
                />
                <Meta
                    name="keywords"
                    content="rust-dd, rust, ai, mathematics, embedded, web, systems, programming"
                />
                <Meta name="robots" content="index, follow" />
                <Meta name="googlebot" content="index, follow" />

                // Facebook
                <Meta property="og:type" content="website" />
                <Meta
                    property="og:title"
                    content="Tech Diaries - The Official Rust-DD Developer Blog"
                />
                <Meta
                    property="og:site_name"
                    content="Tech Diaries - The Official Rust-DD Developer Blog"
                />
                <Meta
                    property="og:description"
                    content="Discover the Rust-DD framework, enabling the application of domain-driven design (DDD) principles in Rust. Write efficient, safe, and clean code with this modern development tool."
                />
                <Meta property="og:url" content="https://rust-dd.com/" />
                <Meta
                    property="og:image"
                    content="https://static.rust-dd.com/rust-dd_custom_bg.png"
                />
                <Meta property="og:image:type" content="image/png" />
                <Meta property="og:image:width" content="1200" />
                <Meta property="og:image:height" content="627" />

                // Twitter
                <Meta name="twitter:card" content="summary_large_image" />
                <Meta
                    name="twitter:title"
                    content="Tech Diaries - The Official Rust-DD Developer Blog"
                />
                <Meta
                    name="twitter:description"
                    content="Discover the Rust-DD framework, enabling the application of domain-driven design (DDD) principles in Rust. Write efficient, safe, and clean code with this modern development tool."
                />
                <Meta name="twitter:site" content="@rust_dd" />
                <Meta name="twitter:url" content="https://rust-dd.com/" />
                <Meta
                    name="twitter:image"
                    content="https://static.rust-dd.com/rust-dd_custom_bg.png"
                />
                <Meta name="twitter:image:alt" content="Rust-DD Framework" />
                <Link rel="preconnect" href="https://fonts.googleapis.com" />
                <Link rel="preconnect" href="https://fonts.gstatic.com" />
            </head>
            <body class="bg-[#1e1e1e]">
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
        <Router>
            <div class="overflow-auto text-white font-poppins">
                <header::Component />
                <main class="container flex flex-col gap-8 px-4 pt-10 pb-14 mx-auto mt-16 max-w-4xl md:px-0">
                    <FlatRoutes fallback=|| {
                        let mut outside_errors = Errors::default();
                        outside_errors.insert_with_default_key(error_template::AppError::NotFound);
                        view! { <error_template::Component outside_errors /> }.into_view()
                    }>
                        <Route path=StaticSegment("") view=home::Component ssr=SsrMode::InOrder />
                        <Route path=StaticSegment("references") view=references::Component />
                        <Route path=StaticSegment("hireus") view=hireus::Component />
                        <Route
                            path=(StaticSegment("post"), ParamSegment("slug"))
                            view=post::Component
                        />
                    </FlatRoutes>
                </main>
                <footer class="fixed right-0 bottom-0 left-0 z-10 py-4 text-center bg-[#1e1e1e]/80 backdrop-blur-md">
                    <p class="text-gray-400">
                        Powered by
                        <a href="https://github.com/rust-dd" class="hover:underline text-[#ffef5c]">
                            {"rust-dd"}
                        </a> {" © "} {Utc::now().year()}
                    </p>
                </footer>
            </div>
        </Router>
    }
}
