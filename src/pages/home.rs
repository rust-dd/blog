use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaCalendarDays, FaClock, FaEye, FaUser},
    Icon,
};

use crate::{app::Route, components::loader, seo, ssr::api::select_posts};

#[component]
pub fn Component() -> Element {
    let posts = use_server_future(select_posts)?;
    let canonical = seo::absolute_url("/");

    rsx! {
        document::Title { "{seo::SITE_NAME}" }
        document::Meta { name: "description", content: seo::SITE_DESCRIPTION }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "googlebot", content: "index, follow" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:title", content: seo::SITE_NAME }
        document::Meta { property: "og:description", content: seo::SITE_DESCRIPTION }
        document::Meta { property: "og:url", content: "{canonical}" }
        document::Meta { property: "og:image", content: seo::DEFAULT_OG_IMAGE }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: seo::SITE_NAME }
        document::Meta { name: "twitter:description", content: seo::SITE_DESCRIPTION }
        document::Meta { name: "twitter:url", content: "{canonical}" }
        document::Meta { name: "twitter:image", content: seo::DEFAULT_OG_IMAGE }
        document::Link { rel: "canonical", href: "{canonical}" }

        SuspenseBoundary {
            fallback: |_| rsx! { loader::Inline { message: "Loading posts...".to_string() } },
            div { class: "mx-auto w-full max-w-6xl font-mono",
                section { class: "relative overflow-hidden p-6 rounded-3xl border border-white/12 nerd-grid md:p-10",
                    div { class: "absolute -top-20 right-0 w-72 h-72 rounded-full bg-[#67e8f9]/10 blur-3xl pointer-events-none" }
                    p { class: "relative z-10 text-xs font-semibold tracking-[0.18em] text-[#67e8f9] uppercase", "Blog" }
                    h1 { class: "relative z-10 mt-4 text-4xl font-semibold tracking-tight text-white md:text-6xl", "From The Grid" }
                    p { class: "relative z-10 mt-3 max-w-3xl text-sm leading-relaxed text-gray-300 md:text-base",
                        "Practical engineering logs on Rust backend systems, architecture, and performance."
                    }
                }

                if let Some(result) = posts.read().as_ref() {
                    match result {
                        Ok(items) => {
                            let featured_posts: Vec<_> = items.iter().take(2).collect();
                            let latest = items
                                .first()
                                .map(|post| post.created_at.clone())
                                .unwrap_or_else(|| "-".to_string());

                            let mut tags: Vec<String> = items
                                .iter()
                                .flat_map(|post| post.tags.iter().cloned())
                                .filter(|tag| !tag.trim().is_empty())
                                .collect();
                            tags.sort_by_key(|tag| tag.to_lowercase());
                            tags.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
                            let top_tags: Vec<_> = tags.into_iter().take(10).collect();

                            rsx! {
                                div { class: "grid gap-8 mt-8 lg:grid-cols-[minmax(0,1fr)_280px]",
                                    div {
                                        if !featured_posts.is_empty() {
                                            section { class: "grid gap-4 md:grid-cols-2",
                                                for post in featured_posts {
                                                    article { class: "overflow-hidden rounded-2xl border border-white/12 bg-black/25 transition-colors duration-200 hover:border-[#67e8f9]/45",
                                                        Link {
                                                            to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                                            class: "block p-5 no-underline",
                                                            p { class: "text-[11px] tracking-[0.16em] text-[#67e8f9] uppercase", "Featured" }
                                                            h2 { class: "mt-2 text-xl leading-tight text-white", "{post.title}" }
                                                            p { class: "mt-2 text-sm leading-relaxed text-gray-300", "{post.summary}" }
                                                            div { class: "flex flex-wrap gap-2 mt-3 text-[11px] text-gray-300",
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-white/15 bg-white/5",
                                                                    Icon { icon: FaUser, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.author.name}"
                                                                }
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-white/15 bg-white/5",
                                                                    Icon { icon: FaClock, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.read_time} min"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        section { class: "mt-8 rounded-2xl border border-white/12 bg-black/20",
                                            div { class: "px-5 py-4 border-b border-white/10",
                                                p { class: "text-xs font-semibold tracking-[0.18em] text-gray-400 uppercase", "All Posts" }
                                            }
                                            div { class: "divide-y divide-white/10",
                                                for post in items.iter() {
                                                    article { class: "px-5 py-5",
                                                        Link {
                                                            to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                                            class: "block no-underline",
                                                            h3 { class: "text-lg leading-tight text-white transition-colors duration-200 hover:text-[#67e8f9] md:text-2xl", "{post.title}" }
                                                            p { class: "mt-2 text-sm leading-relaxed text-gray-300", "{post.summary}" }
                                                            div { class: "flex flex-wrap gap-2 mt-3 text-[11px] text-gray-300",
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-white/15 bg-white/5",
                                                                    Icon { icon: FaCalendarDays, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.created_at}"
                                                                }
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-white/15 bg-white/5",
                                                                    Icon { icon: FaClock, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.read_time} min"
                                                                }
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-white/15 bg-white/5",
                                                                    Icon { icon: FaEye, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.total_views} views"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    aside { class: "space-y-4 h-fit lg:sticky lg:top-24",
                                        div { class: "overflow-hidden relative p-4 rounded-2xl border border-white/12 nerd-grid",
                                            p { class: "text-xs font-semibold tracking-[0.18em] text-gray-400 uppercase", "blog.sys" }
                                            div { class: "mt-3 space-y-2 text-xs text-gray-300",
                                                p { span { class: "text-[#67e8f9]", "$ " } "posts = " span { class: "text-white", "{items.len()}" } }
                                                p { span { class: "text-[#67e8f9]", "$ " } "latest = " span { class: "text-white", "{latest}" } }
                                                p { span { class: "text-[#67e8f9]", "$ " } "stack = " span { class: "text-white", "rust / dioxus / axum" } }
                                            }
                                        }

                                        if !top_tags.is_empty() {
                                            div { class: "p-4 rounded-2xl border border-white/12 bg-black/20",
                                                p { class: "text-xs font-semibold tracking-[0.18em] text-gray-400 uppercase", "topics" }
                                                div { class: "flex flex-wrap gap-2 mt-3",
                                                    for tag in top_tags {
                                                        span { class: "rounded-full border border-white/15 px-2.5 py-1 text-[11px] text-gray-200", "#{tag}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => rsx! {
                            div { class: "mt-8 text-red-300", "Failed to load posts: {err}" }
                        },
                    }
                }
            }
        }
    }
}
