use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaCalendarDays, FaClock, FaEye, FaUser},
    Icon,
};
use std::collections::BTreeMap;

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
                section { class: "relative overflow-hidden p-5 rounded-3xl border border-slate-200 bg-white/92 shadow-sm sm:p-7 md:p-10",
                    div { class: "absolute -top-20 right-0 w-72 h-72 rounded-full bg-slate-300/20 blur-3xl pointer-events-none" }
                    p { class: "relative z-10 text-[11px] font-semibold tracking-[0.2em] text-slate-700 uppercase", "Blog" }
                    h1 { class: "relative z-10 mt-3 text-3xl font-semibold tracking-tight text-slate-900 sm:text-4xl md:text-5xl", "Engineering Notes" }
                    p { class: "relative z-10 mt-3 max-w-3xl text-sm leading-relaxed text-slate-600 md:text-base",
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

                            let mut tag_counts: BTreeMap<String, usize> = BTreeMap::new();
                            for post in items.iter() {
                                for tag in post.tags.iter() {
                                    let normalized = tag.trim().to_lowercase();
                                    if normalized.is_empty() {
                                        continue;
                                    }
                                    *tag_counts.entry(normalized).or_insert(0) += 1;
                                }
                            }

                            let mut top_tags: Vec<(String, usize)> = tag_counts.into_iter().collect();
                            top_tags.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
                            let top_tags: Vec<_> = top_tags.into_iter().take(18).collect();

                            rsx! {
                                div { class: "grid gap-6 mt-6 md:gap-8 lg:grid-cols-[minmax(0,1fr)_280px]",
                                    div {
                                        if !featured_posts.is_empty() {
                                            section { class: "grid gap-4 md:grid-cols-2",
                                                for post in featured_posts {
                                                    article { class: "overflow-hidden rounded-2xl border border-slate-200 bg-white/90 shadow-sm transition-colors duration-200 hover:border-slate-400",
                                                        Link {
                                                            to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                                            class: "block p-4 no-underline sm:p-5",
                                                            p { class: "text-[10px] font-semibold tracking-[0.16em] text-slate-700 uppercase", "Featured" }
                                                            h2 { class: "mt-2 text-lg leading-tight text-slate-900 sm:text-xl", "{post.title}" }
                                                            p { class: "mt-2 text-sm leading-relaxed text-slate-600", "{post.summary}" }
                                                            div { class: "flex flex-wrap gap-2 mt-3 text-[11px] text-slate-600",
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-slate-200 bg-slate-50",
                                                                    Icon { icon: FaUser, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.author.name}"
                                                                }
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-slate-200 bg-slate-50",
                                                                    Icon { icon: FaClock, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.read_time} min"
                                                                }
                                                                span { class: "hidden sm:inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-slate-200 bg-slate-50",
                                                                    Icon { icon: FaEye, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.total_views} views"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        section { class: "mt-6 rounded-2xl border border-slate-200 bg-white/90 shadow-sm",
                                            div { class: "px-4 py-3 border-b border-slate-200 sm:px-5 sm:py-4",
                                                p { class: "text-[11px] font-semibold tracking-[0.18em] text-slate-500 uppercase", "All Posts" }
                                            }
                                            div { class: "divide-y divide-slate-200",
                                                for post in items.iter() {
                                                    article { class: "px-4 py-4 sm:px-5 sm:py-5",
                                                        Link {
                                                            to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                                            class: "block no-underline",
                                                            h3 { class: "text-lg leading-tight text-slate-900 transition-colors duration-200 hover:text-slate-700 sm:text-xl md:text-2xl", "{post.title}" }
                                                            p { class: "mt-2 text-sm leading-relaxed text-slate-600", "{post.summary}" }
                                                            div { class: "flex flex-wrap gap-2 mt-3 text-[11px] text-slate-600",
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-slate-200 bg-slate-50",
                                                                    Icon { icon: FaCalendarDays, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.created_at}"
                                                                }
                                                                span { class: "inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-slate-200 bg-slate-50",
                                                                    Icon { icon: FaClock, width: 11, height: 11, fill: "currentColor" }
                                                                    "{post.read_time} min"
                                                                }
                                                                span { class: "hidden sm:inline-flex gap-1.5 items-center px-2 py-1 rounded-full border border-slate-200 bg-slate-50",
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
                                        div { class: "overflow-hidden relative p-4 rounded-2xl border border-slate-200 bg-white/90 shadow-sm",
                                            p { class: "text-[11px] font-semibold tracking-[0.18em] text-slate-500 uppercase", "blog.sys" }
                                            div { class: "mt-3 space-y-2 text-xs text-slate-600",
                                                p { span { class: "text-slate-700", "$ " } "posts = " span { class: "text-slate-900", "{items.len()}" } }
                                                p { span { class: "text-slate-700", "$ " } "latest = " span { class: "text-slate-900", "{latest}" } }
                                                p { span { class: "text-slate-700", "$ " } "stack = " span { class: "text-slate-900", "rust / dioxus / axum" } }
                                            }
                                        }

                                        if !top_tags.is_empty() {
                                            div { class: "p-4 rounded-2xl border border-slate-200 bg-white/90 shadow-sm",
                                                p { class: "text-[11px] font-semibold tracking-[0.18em] text-slate-500 uppercase", "topics" }
                                                div { class: "flex flex-wrap gap-2 mt-3",
                                                    for (tag, count) in top_tags {
                                                        span { class: "inline-flex items-center gap-1 rounded-full border border-slate-200 bg-slate-50 px-2.5 py-1 text-[11px] text-slate-700",
                                                            "#{tag}"
                                                            span { class: "text-[10px] text-slate-500", "({count})" }
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        div { class: "overflow-hidden relative p-4 rounded-2xl border-2 border-slate-900 bg-slate-900 text-white shadow-md",
                                            div { class: "absolute inset-0 bg-[radial-gradient(circle_at_top_right,rgba(255,255,255,0.08),transparent_52%)] pointer-events-none" }
                                            p { class: "relative text-[11px] font-semibold tracking-[0.18em] text-slate-300 uppercase", "Try out our fun projects" }
                                            div { class: "mt-3 space-y-2 text-sm",
                                                a {
                                                    href: "https://shrtn.ink/",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "relative flex items-center justify-between rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-slate-100 transition-colors duration-200 hover:bg-slate-700",
                                                    span { "shrtn.ink" }
                                                    span { class: "text-slate-400", "↗" }
                                                }
                                                a {
                                                    href: "https://stochasticlab.cloud/",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "relative flex items-center justify-between rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-slate-100 transition-colors duration-200 hover:bg-slate-700",
                                                    span { "stochasticlab.cloud" }
                                                    span { class: "text-slate-400", "↗" }
                                                }
                                                a {
                                                    href: "https://tryrust.org/",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "relative flex items-center justify-between rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-slate-100 transition-colors duration-200 hover:bg-slate-700",
                                                    span { "tryrust.org" }
                                                    span { class: "text-slate-400", "↗" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => rsx! {
                            div { class: "mt-8 text-red-600", "Failed to load posts: {err}" }
                        },
                    }
                }
            }
        }
    }
}
