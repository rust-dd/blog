use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;
use std::collections::BTreeMap;

use crate::{app::Route, seo, ssr::api::select_posts};

#[component]
pub fn Component(query: String) -> Element {
    let _ = query;
    let posts = use_server_future(select_posts)?;

    // A 5xx keeps crawlers from indexing the error state as the home page.
    if let Some(Err(err)) = posts.read().as_ref() {
        FullstackContext::commit_error_status(err.clone());
    }

    rsx! {
        seo::PageMeta {
            title: seo::HOME_TITLE,
            description: seo::SITE_DESCRIPTION,
            path: "/",
        }
        seo::JsonLd { value: seo::website_graph() }

        crate::components::shell::Sidebar {}
        main { id: "main", class: "shell-main",
            div { class: "w-full font-mono",
                section { class: "animate-rise py-6 sm:py-8",
                    p { class: "text-xs text-faint",
                        span { class: "text-accent", "//" }
                        " engineering notes"
                    }
                    h1 { class: "mt-3 text-4xl font-semibold leading-[1.05] tracking-tight text-fg sm:text-5xl md:text-6xl",
                        "Practical "
                        span { class: "text-accent", "Rust" }
                        " Engineering"
                        span { class: "ml-1.5 inline-block h-7 w-2.5 animate-pulse bg-accent align-middle sm:h-9 sm:w-3" }
                    }
                    p { class: "mt-4 max-w-2xl text-sm leading-relaxed text-muted sm:text-base",
                        "Logs on Rust backend systems, architecture, and performance."
                    }
                }

                if let Some(result) = posts.read().as_ref() {
                    match result {
                        Ok(items) => {
                            let featured_posts: Vec<_> = items.iter().take(2).collect();
                            let latest = items
                                .first()
                                .map(|post| post.published_on())
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
                                div { class: "mt-4 border-y border-dashed border-border py-3 text-xs text-muted",
                                    div { class: "flex flex-wrap gap-x-4 gap-y-1",
                                        span { "posts: " span { class: "text-fg", "{items.len()}" } }
                                        span { class: "hidden sm:inline", "|" }
                                        span { "latest: " span { class: "text-fg", "{latest}" } }
                                        span { class: "hidden sm:inline", "|" }
                                        span { "stack: " span { class: "text-fg", "rust/dioxus/axum" } }
                                    }
                                }

                                if !tag_names.is_empty() {
                                    div { class: "mt-4 text-xs text-muted",
                                        span { class: "text-faint", "use " }
                                        span { class: "text-muted", "topics" }
                                        span { class: "text-faint", "::" }
                                        span { class: "text-faint", "{{" }
                                        span { class: "text-fg",
                                            {tag_names.join(", ")}
                                        }
                                        span { class: "text-faint", "}};" }
                                    }
                                }

                                if !featured_posts.is_empty() {
                                    section { class: "mt-8",
                                        p { class: "text-xs text-faint", "// featured" }
                                        div { class: "mt-3 flex flex-col gap-4",
                                            for post in featured_posts {
                                                article { class: "group rounded-lg border border-border bg-surface p-4 transition-colors duration-200 hover:border-accent sm:p-5",
                                                    Link {
                                                        to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                                        class: "flex flex-col gap-3 no-underline sm:flex-row sm:items-baseline sm:justify-between",
                                                        div { class: "min-w-0",
                                                            h2 { class: "text-lg leading-tight text-fg transition-colors duration-200 group-hover:text-accent sm:text-xl", "{post.title}" }
                                                            p { class: "mt-2 text-sm leading-relaxed text-muted", "{post.summary}" }
                                                        }
                                                        p { class: "shrink-0 text-xs text-faint sm:text-right",
                                                            "{post.read_time}min · {post.total_views} views"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                section { class: "mt-8",
                                    p { class: "text-xs text-faint", "// all posts" }
                                    div { class: "mt-3 rounded-lg border border-border bg-surface",
                                        div { class: "hidden border-b border-dashed border-border px-4 py-2 text-[11px] font-semibold text-faint sm:grid sm:grid-cols-[120px_1fr_70px_70px]",
                                            span { "date" }
                                            span { "title" }
                                            span { class: "text-right", "read" }
                                            span { class: "text-right", "views" }
                                        }
                                        div { class: "divide-y divide-border",
                                            for post in items.iter() {
                                                Link {
                                                    to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                                    class: "block px-4 py-3 no-underline transition-colors duration-150 hover:bg-surface-2",
                                                    div { class: "hidden sm:grid sm:grid-cols-[120px_1fr_70px_70px] sm:items-center",
                                                        span { class: "text-xs text-faint", "{post.published_on()}" }
                                                        span { class: "truncate pr-4 text-sm text-fg", "{post.title}" }
                                                        span { class: "text-right text-xs text-faint", "{post.read_time}min" }
                                                        span { class: "text-right text-xs text-faint", "{post.total_views}" }
                                                    }
                                                    div { class: "sm:hidden",
                                                        p { class: "text-sm text-fg", "{post.title}" }
                                                        p { class: "mt-1 text-xs text-faint",
                                                            "{post.published_on()} · {post.read_time}min · {post.total_views} views"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => rsx! {
                            div { class: "mt-8 text-red-500", "Failed to load posts: {err}" }
                        },
                    }
                }
            }
        }
    }
}
