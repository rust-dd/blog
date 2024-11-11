use crate::error_template::{AppError, ErrorTemplate};
use chrono::{Datelike, Utc};
use icondata as i;
use leptos::*;
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::*;

use crate::{home, post};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <body class="bg-[#1e1e1e]">
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
                <Link
                    href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&family=Poppins:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap"
                    rel="stylesheet"
                />
            </body>
        </html>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <div class="overflow-auto text-white font-poppins">
                <header class="fixed top-0 right-0 left-0 z-10 py-6 px-4 md:px-6 bg-[#1e1e1e]/80 backdrop-blur-md">
                    <div class="container mx-auto max-w-5xl">
                        <div class="flex flex-row justify-between items-center text-white">
                            <a
                                href="/"
                                class="text-xl font-bold transition-all duration-500 sm:text-3xl hover:text-[#ffef5c]"
                            >
                                rust-dd.com
                            </a>
                            <div class="flex flex-row gap-3 items-center h-10">
                                <a
                                    href="https://github.com/rust-dd/blog"
                                    rel="noopener noreferrer"
                                    target="_blank"
                                    aria-label="GitHub"
                                >
                                    <Icon
                                        icon=i::IoLogoGithub
                                        class="transition-all duration-500 size-6 hover:text-[#ffef5c]"
                                    />
                                </a>
                                <a
                                    href="https://x.com/rust_dd"
                                    rel="noopener noreferrer"
                                    target="_blank"
                                    aria-label="X"
                                >
                                    <Icon
                                        icon=i::FaXTwitterBrands
                                        class="transition-all duration-500 size-6 hover:text-[#ffef5c]"
                                    />
                                </a>
                                <a
                                    href="https://www.linkedin.com/company/rust-dd"
                                    rel="noopener noreferrer"
                                    target="_blank"
                                    aria-label="LinkedIn"
                                >
                                    <Icon
                                        icon=i::FaLinkedinBrands
                                        class="transition-all duration-500 size-6 hover:text-[#ffef5c]"
                                    />
                                </a>
                                <a
                                    href="/rss.xml"
                                    rel="noopener noreferrer"
                                    target="_blank"
                                    aria-label="RSS"
                                >
                                    <Icon
                                        icon=i::IoLogoRss
                                        class="transition-all duration-500 size-6 hover:text-[#ffef5c]"
                                    />
                                </a>
                            </div>
                        </div>
                    </div>
                </header>
                <main class="container flex flex-col gap-8 px-4 pt-10 pb-14 mx-auto mt-16 max-w-4xl md:px-0">
                    <Routes>
                        <Route path="/" view=move || view! { <home::Component /> } />
                        <Route path="/post/:slug/" view=move || view! { <post::Component /> } />
                    </Routes>
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
