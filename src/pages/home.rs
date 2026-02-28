use dioxus::prelude::*;
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
            div { class: "w-full font-mono",
                // Hero
                section { class: "py-4",
                    p { class: "text-xs text-slate-400", "// engineering notes" }
                    h1 { class: "mt-2 text-3xl font-semibold tracking-tight text-slate-900 sm:text-4xl md:text-5xl",
                        "Practical Rust Engineering"
                    }
                    p { class: "mt-3 max-w-2xl text-sm leading-relaxed text-slate-600",
                        "Logs on Rust backend systems, architecture, and performance."
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
                            let top_tags: Vec<_> = top_tags.into_iter().take(12).collect();
                            let tag_names: Vec<String> = top_tags.iter().map(|(name, _)| name.clone()).collect();

                            rsx! {
                                // Status bar
                                div { class: "mt-4 border-y border-dashed border-slate-300 py-3 text-xs text-slate-500",
                                    div { class: "flex flex-wrap gap-x-4 gap-y-1",
                                        span { "posts: " span { class: "text-slate-700", "{items.len()}" } }
                                        span { class: "hidden sm:inline", "|" }
                                        span { "latest: " span { class: "text-slate-700", "{latest}" } }
                                        span { class: "hidden sm:inline", "|" }
                                        span { "stack: " span { class: "text-slate-700", "rust/dioxus/axum" } }
                                    }
                                }

                                // Topics
                                if !tag_names.is_empty() {
                                    div { class: "mt-4 text-xs text-slate-500",
                                        span { class: "text-slate-400", "use " }
                                        span { class: "text-slate-500", "topics" }
                                        span { class: "text-slate-400", "::" }
                                        span { class: "text-slate-400", "{{" }
                                        span { class: "text-slate-600",
                                            {tag_names.join(", ")}
                                        }
                                        span { class: "text-slate-400", "}};" }
                                    }
                                }

                                // Featured posts
                                if !featured_posts.is_empty() {
                                    section { class: "mt-8",
                                        p { class: "text-xs text-slate-400", "// featured" }
                                        div { class: "mt-3 grid gap-4 md:grid-cols-2",
                                            for post in featured_posts {
                                                article { class: "rounded-lg border border-slate-200 bg-white p-4 transition-colors duration-200 hover:border-slate-400 sm:p-5",
                                                    Link {
                                                        to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                                        class: "block no-underline",
                                                        h2 { class: "text-lg leading-tight text-slate-900 sm:text-xl", "{post.title}" }
                                                        p { class: "mt-2 text-sm leading-relaxed text-slate-600", "{post.summary}" }
                                                        p { class: "mt-3 text-xs text-slate-400",
                                                            "author={post.author.name} read={post.read_time}min views={post.total_views}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // All posts (ls -la style)
                                section { class: "mt-8",
                                    p { class: "text-xs text-slate-400", "// all posts" }
                                    div { class: "mt-3 rounded-lg border border-slate-200 bg-white",
                                        // Table header
                                        div { class: "hidden border-b border-dashed border-slate-200 px-4 py-2 text-[11px] font-semibold text-slate-400 sm:grid sm:grid-cols-[120px_1fr_70px_70px]",
                                            span { "date" }
                                            span { "title" }
                                            span { class: "text-right", "read" }
                                            span { class: "text-right", "views" }
                                        }
                                        div { class: "divide-y divide-slate-100",
                                            for post in items.iter() {
                                                Link {
                                                    to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                                    class: "block px-4 py-3 no-underline transition-colors duration-150 hover:bg-slate-50",
                                                    // Desktop row
                                                    div { class: "hidden sm:grid sm:grid-cols-[120px_1fr_70px_70px] sm:items-center",
                                                        span { class: "text-xs text-slate-400", "{post.created_at}" }
                                                        span { class: "text-sm text-slate-900 truncate pr-4", "{post.title}" }
                                                        span { class: "text-xs text-slate-400 text-right", "{post.read_time}min" }
                                                        span { class: "text-xs text-slate-400 text-right", "{post.total_views}" }
                                                    }
                                                    // Mobile
                                                    div { class: "sm:hidden",
                                                        p { class: "text-sm text-slate-900", "{post.title}" }
                                                        p { class: "mt-1 text-xs text-slate-400",
                                                            "{post.created_at} · {post.read_time}min · {post.total_views} views"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Projects
                                section { class: "mt-8",
                                    p { class: "text-xs text-slate-400", "// projects" }
                                    div { class: "mt-3 grid gap-4 sm:grid-cols-3",
                                        ProjectCard {
                                            name: "shrtn.ink",
                                            description: "Fast URL shortener",
                                            url: "https://shrtn.ink/"
                                        }
                                        ProjectCard {
                                            name: "stochasticlab",
                                            description: "Cloud compute platform",
                                            url: "https://stochasticlab.cloud/"
                                        }
                                        ProjectCard {
                                            name: "tryrust.org",
                                            description: "Interactive Rust playground",
                                            url: "https://tryrust.org/"
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

#[component]
fn ProjectCard(name: &'static str, description: &'static str, url: &'static str) -> Element {
    rsx! {
        a {
            href: "{url}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "group flex items-center justify-between rounded-lg border border-slate-200 bg-white px-4 py-3 text-sm transition-colors duration-200 hover:border-slate-400",
            div {
                p { class: "font-medium text-slate-900", "{name}" }
                p { class: "text-xs text-slate-500", "{description}" }
            }
            span { class: "text-slate-300 transition-colors duration-200 group-hover:text-slate-500", ">" }
        }
    }
}
