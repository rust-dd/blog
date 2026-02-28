use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaCalendarDays, FaClock, FaEye, FaUser},
    Icon,
};

use crate::{
    app::Route,
    components::loader,
    seo,
    ssr::api::{increment_views, select_post},
};

#[component]
pub fn Component(slug: String) -> Element {
    let post = use_server_future(move || {
        let slug = slug.clone();
        async move { select_post(slug).await }
    })?;
    let mut view_counted = use_signal(|| false);

    use_effect(move || {
        if cfg!(not(debug_assertions)) && !*view_counted.read() {
            if let Some(Ok(post)) = post.read().as_ref() {
                view_counted.set(true);
                let id = post.id.id.to_string();
                spawn(async move {
                    let _ = increment_views(id).await;
                });
            }
        }
    });

    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! { loader::Inline { message: "Loading post...".to_string() } },
            if let Some(result) = post.read().as_ref() {
                match result {
                    Ok(post) => {
                        let canonical = seo::absolute_url(&format!(
                            "/post/{}",
                            post.slug.clone().unwrap_or_default()
                        ));
                        let og_image = post
                            .header_image
                            .clone()
                            .unwrap_or_else(|| seo::DEFAULT_OG_IMAGE.to_string());
                        rsx! {
                            document::Title { "{post.title}" }
                            document::Meta { name: "description", content: "{post.summary}" }
                            document::Meta { name: "robots", content: "index, follow" }
                            document::Meta { name: "googlebot", content: "index, follow" }
                            document::Meta { property: "og:type", content: "article" }
                            document::Meta { property: "og:title", content: "{post.title}" }
                            document::Meta { property: "og:description", content: "{post.summary}" }
                            document::Meta { property: "og:url", content: "{canonical}" }
                            document::Meta { property: "og:image", content: "{og_image}" }
                            document::Meta { property: "og:image:alt", content: "{post.title}" }
                            document::Meta { name: "twitter:card", content: "summary_large_image" }
                            document::Meta { name: "twitter:title", content: "{post.title}" }
                            document::Meta { name: "twitter:description", content: "{post.summary}" }
                            document::Meta { name: "twitter:url", content: "{canonical}" }
                            document::Meta { name: "twitter:image", content: "{og_image}" }
                            document::Meta { name: "twitter:image:alt", content: "{post.title}" }
                            document::Link { rel: "canonical", href: "{canonical}" }

                            div { class: "relative w-full mx-auto max-w-4xl",
                                div { class: "overflow-hidden absolute inset-0 pointer-events-none",
                                    div { class: "absolute -top-10 -right-16 w-64 h-64 bg-[#ffef5c]/5 rounded-full blur-3xl" }
                                    div { class: "absolute top-40 -left-16 w-72 h-72 bg-white/[0.04] rounded-full blur-3xl" }
                                }

                                article { class: "relative z-10 overflow-hidden p-5 rounded-3xl border border-white/10 bg-gradient-to-b from-white/[0.07] to-white/[0.02] md:p-8",
                                    div { class: "inline-flex gap-2.5 items-center px-3 py-1 mb-4 text-[11px] font-semibold tracking-[0.14em] text-[#ffef5c] uppercase rounded-full border bg-[#ffef5c]/10 border-[#ffef5c]/30",
                                        span { "Article" }
                                    }

                                    p { class: "text-3xl font-semibold leading-tight text-white md:text-5xl", "{post.title}" }

                                    div { class: "flex flex-wrap gap-2.5 items-center mt-5 text-xs text-gray-300",
                                        if let Some(github) = post.author.github.clone() {
                                            a {
                                                href: "{github}",
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                class: "inline-flex gap-1.5 items-center px-3 py-1.5 rounded-full border bg-white/5 border-white/10 hover:border-[#ffef5c]/30 hover:text-[#ffef5c] transition-colors duration-300",
                                                Icon { icon: FaUser, width: 13, height: 13, fill: "currentColor" }
                                                span { "{post.author.name}" }
                                            }
                                        } else {
                                            div { class: "inline-flex gap-1.5 items-center px-3 py-1.5 rounded-full border bg-white/5 border-white/10",
                                                Icon { icon: FaUser, width: 13, height: 13, fill: "currentColor" }
                                                span { "{post.author.name}" }
                                            }
                                        }

                                        div { class: "inline-flex gap-1.5 items-center px-3 py-1.5 rounded-full border bg-white/5 border-white/10",
                                            Icon { icon: FaCalendarDays, width: 13, height: 13, fill: "currentColor" }
                                            span { "{post.created_at}" }
                                        }
                                        div { class: "inline-flex gap-1.5 items-center px-3 py-1.5 rounded-full border bg-white/5 border-white/10",
                                            Icon { icon: FaClock, width: 13, height: 13, fill: "currentColor" }
                                            span { "{post.read_time} min read" }
                                        }
                                        div { class: "inline-flex gap-1.5 items-center px-3 py-1.5 rounded-full border bg-white/5 border-white/10",
                                            Icon { icon: FaEye, width: 13, height: 13, fill: "currentColor" }
                                            span { "{post.total_views} views" }
                                        }
                                    }

                                    if let Some(image) = post.header_image.clone() {
                                        div { class: "overflow-hidden mt-7 rounded-2xl border border-white/10 bg-black/30",
                                            img {
                                                src: "{image}",
                                                alt: "{post.title}",
                                                class: "object-cover w-full max-h-[440px]"
                                            }
                                        }
                                    }
                                }

                                div {
                                    class: "relative z-10 p-4 mt-6 rounded-3xl border border-white/10 bg-[#1f1f1f]/80 md:p-8",
                                    div {
                                        class: "prose mx-auto max-w-none prose-h3:text-white prose-h4:text-white prose-code:before:content-none prose-th:text-white prose-li:marker:text-white prose-code:after:content-none prose-pre:bg-transparent prose-pre:rounded-lg prose-pre:p-0 prose-code:text-[#ffef5c] prose-strong:text-white prose-table:text-white prose-thead:text-white prose-li:text-white prose-ol:text-white prose-h1:text-white prose-h1:text-3xl prose-h2:text-white prose-h2:text-2xl prose-ul:text-white prose-p:text-white prose-a:text-[#ffef5c] prose-pre:whitespace-pre-wrap prose-pre:break-all prose-pre:overflow-auto",
                                        dangerous_inner_html: "{post.body}"
                                    }
                                }

                                if post.show_cta {
                                    div { class: "relative z-10 mt-8 mb-10",
                                        div { class: "flex flex-col gap-6 justify-between items-center p-8 rounded-2xl border border-[#ffef5c]/30 bg-gradient-to-r from-[#2e2e2e] to-[#242424] md:flex-row",
                                            div { class: "text-center md:text-left",
                                                p { class: "mb-2 text-2xl font-bold text-[#ffef5c] md:text-3xl", "Need Rust Expertise?" }
                                                p { class: "max-w-md text-gray-300", "Our team of Rust developers is ready to bring your high-performance projects to life." }
                                            }
                                            Link {
                                                to: Route::HireUs {},
                                                class: "rounded-full bg-[#ffef5c] px-6 py-3 text-lg font-semibold text-gray-900 transition-all duration-300 hover:brightness-95",
                                                "Hire Rust Developers"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => rsx! {
                        section { class: "mx-auto max-w-3xl text-center pt-24",
                            h1 { class: "text-3xl font-semibold text-red-300", "Failed to load post" }
                            p { class: "mt-4 text-gray-300", "{err}" }
                            Link {
                                to: Route::Home {},
                                class: "inline-flex mt-8 text-[#ffef5c] hover:underline",
                                "Go back home"
                            }
                        }
                    },
                }
            }
        }
    }
}
