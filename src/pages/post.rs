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

                            div { class: "mx-auto w-full max-w-6xl font-mono",
                                Link {
                                    to: Route::Home {},
                                    class: "inline-flex gap-2 items-center text-sm text-gray-300 transition-colors duration-200 hover:text-[#67e8f9]",
                                    span { "←" }
                                    span { "Back to blog" }
                                }

                                div { class: "grid gap-8 mt-4 lg:grid-cols-[minmax(0,1fr)_280px]",
                                    article {
                                        section { class: "relative overflow-hidden p-6 rounded-3xl border border-white/12 nerd-grid md:p-10",
                                            div { class: "absolute -top-20 right-0 w-72 h-72 rounded-full bg-[#67e8f9]/10 blur-3xl pointer-events-none" }
                                            p { class: "relative z-10 text-xs font-semibold tracking-[0.18em] text-[#67e8f9] uppercase", "Article" }
                                            h1 { class: "relative z-10 mt-4 max-w-4xl text-3xl font-semibold leading-tight text-white md:text-6xl", "{post.title}" }
                                            p { class: "relative z-10 mt-4 max-w-3xl text-sm leading-relaxed text-gray-300 md:text-base", "{post.summary}" }

                                            div { class: "relative z-10 flex flex-wrap gap-2 mt-5 text-xs text-gray-300",
                                                div { class: "inline-flex items-center gap-2 rounded-full border border-white/15 bg-black/25 px-2.5 py-1.5",
                                                    Icon { icon: FaUser, width: 12, height: 12, fill: "currentColor" }
                                                    span { "{post.author.name}" }
                                                }
                                                div { class: "inline-flex items-center gap-2 rounded-full border border-white/15 bg-black/25 px-2.5 py-1.5",
                                                    Icon { icon: FaCalendarDays, width: 12, height: 12, fill: "currentColor" }
                                                    span { "{post.created_at}" }
                                                }
                                                div { class: "inline-flex items-center gap-2 rounded-full border border-white/15 bg-black/25 px-2.5 py-1.5",
                                                    Icon { icon: FaClock, width: 12, height: 12, fill: "currentColor" }
                                                    span { "{post.read_time} min read" }
                                                }
                                                div { class: "inline-flex items-center gap-2 rounded-full border border-white/15 bg-black/25 px-2.5 py-1.5",
                                                    Icon { icon: FaEye, width: 12, height: 12, fill: "currentColor" }
                                                    span { "{post.total_views} views" }
                                                }
                                            }

                                            if let Some(image) = post.header_image.clone() {
                                                div { class: "overflow-hidden relative z-10 mt-8 rounded-3xl border border-white/12 bg-[#0f0f0f]",
                                                    img {
                                                        src: "{image}",
                                                        alt: "{post.title}",
                                                        class: "object-cover w-full max-h-[520px]"
                                                    }
                                                }
                                            }
                                        }

                                        div { class: "p-5 mt-8 rounded-3xl border border-white/12 bg-[#161616] md:p-8",
                                            div {
                                                class: "prose prose-lg max-w-none prose-h3:text-white prose-h4:text-white prose-code:before:content-none prose-th:text-white prose-li:marker:text-white prose-code:after:content-none prose-pre:bg-[#101010] prose-pre:rounded-xl prose-pre:px-4 prose-pre:py-3 prose-code:text-[#67e8f9] prose-code:bg-white/10 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-strong:text-white prose-table:text-white prose-thead:text-white prose-li:text-white prose-ol:text-white prose-h1:text-white prose-h1:text-3xl prose-h2:text-white prose-h2:text-2xl prose-ul:text-white prose-p:text-white prose-a:text-[#67e8f9] prose-p:leading-8 prose-li:leading-7 prose-pre:whitespace-pre-wrap prose-pre:break-all prose-pre:overflow-auto",
                                                dangerous_inner_html: "{post.body}"
                                            }
                                        }
                                    }

                                    aside { class: "space-y-4 h-fit lg:sticky lg:top-24",
                                        div { class: "overflow-hidden relative p-4 rounded-2xl border border-white/12 nerd-grid",
                                            p { class: "text-xs font-semibold tracking-[0.18em] text-gray-400 uppercase", "meta" }
                                            div { class: "mt-3 space-y-2 text-xs text-gray-300",
                                                p { "author: " span { class: "text-white", "{post.author.name}" } }
                                                p { "published: " span { class: "text-white", "{post.created_at}" } }
                                                p { "read: " span { class: "text-white", "{post.read_time} min" } }
                                                p { "views: " span { class: "text-white", "{post.total_views}" } }
                                            }
                                        }

                                        if !post.tags.is_empty() {
                                            div { class: "p-4 rounded-2xl border border-white/12 bg-black/20",
                                                p { class: "text-xs font-semibold tracking-[0.18em] text-gray-400 uppercase", "tags" }
                                                div { class: "flex flex-wrap gap-2 mt-3",
                                                    for tag in post.tags.iter().take(10) {
                                                        span { class: "rounded-full border border-white/15 px-2.5 py-1 text-[11px] text-gray-200", "#{tag}" }
                                                    }
                                                }
                                            }
                                        }

                                        if post.show_cta {
                                            div { class: "p-4 rounded-2xl border border-[#67e8f9]/30 bg-[#0e2229]",
                                                p { class: "text-lg font-semibold text-[#67e8f9]", "Need Rust Expertise?" }
                                                p { class: "mt-2 text-sm text-gray-300", "Build your next production Rust system with us." }
                                                Link {
                                                    to: Route::HireUs {},
                                                    class: "inline-flex mt-4 items-center justify-center rounded-full bg-[#67e8f9] px-4 py-2 text-sm font-semibold text-[#062029] transition-all duration-200 hover:brightness-95",
                                                    "Hire Rust Developers"
                                                }
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
                                class: "inline-flex mt-8 text-[#67e8f9] hover:underline",
                                "Go back home"
                            }
                        }
                    },
                }
            }
        }
    }
}
