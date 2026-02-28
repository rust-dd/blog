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
            div { class: "relative container max-w-4xl mx-auto",
                div { class: "overflow-hidden absolute inset-0 pointer-events-none",
                    div { class: "absolute -top-10 -right-16 w-64 h-64 bg-[#ffef5c]/5 rounded-full blur-3xl" }
                    div { class: "absolute top-1/3 -left-20 w-72 h-72 bg-white/[0.04] rounded-full blur-3xl" }
                }
                section { class: "relative mb-8",
                    p { class: "text-xs uppercase tracking-[0.18em] text-[#ffef5c]/80", "Latest from Rust-DD" }
                    h1 { class: "mt-3 text-3xl font-bold text-white md:text-4xl", "Fresh Rust Articles & Notes" }
                    p { class: "mt-2 max-w-2xl text-sm text-gray-300 md:text-base", "Short practical posts on Rust backend, architecture, and performance." }
                }
                if let Some(result) = posts.read().as_ref() {
                    match result {
                        Ok(items) => rsx! {
                            div { class: "relative flex flex-col gap-5",
                                for post in items {
                                    article { class: "group relative overflow-hidden rounded-2xl border border-white/10 bg-gradient-to-br from-white/[0.07] to-white/[0.02] transition-all duration-300 hover:-translate-y-0.5 hover:border-[#ffef5c]/40 hover:shadow-[0_18px_45px_-28px_rgba(255,239,92,0.55)]",
                                        div { class: "absolute top-0 right-0 left-0 h-px bg-gradient-to-r from-transparent via-[#ffef5c]/45 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" }
                                        Link {
                                            to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                            class: "block p-5 no-underline md:p-6",
                                            div { class: "relative z-10 flex flex-col gap-4",
                                                div { class: "flex gap-3 justify-between items-start",
                                                    p { class: "text-xl font-semibold leading-tight text-white transition-colors duration-300 line-clamp-2 group-hover:text-[#ffef5c]", "{post.title}" }
                                                    span { class: "shrink-0 rounded-full border border-[#ffef5c]/30 bg-[#ffef5c]/10 px-2.5 py-1 text-[10px] font-semibold tracking-wider text-[#ffef5c] uppercase", "Post" }
                                                }
                                                p { class: "text-sm font-light leading-relaxed text-gray-300 md:text-[15px]", "{post.summary}" }

                                                div { class: "flex flex-wrap gap-2.5 justify-start items-center text-[11px] text-gray-300",
                                                    div { class: "inline-flex gap-1.5 items-center px-2.5 py-1 rounded-full border bg-white/5 border-white/10",
                                                        Icon { icon: FaClock, width: 12, height: 12, fill: "currentColor" }
                                                        p { "{post.read_time} min read" }
                                                    }
                                                    div { class: "inline-flex gap-1.5 items-center px-2.5 py-1 rounded-full border bg-white/5 border-white/10",
                                                        Icon { icon: FaEye, width: 12, height: 12, fill: "currentColor" }
                                                        p { "{post.total_views} views" }
                                                    }
                                                    div { class: "inline-flex gap-1.5 items-center px-2.5 py-1 rounded-full border bg-white/5 border-white/10",
                                                        Icon { icon: FaCalendarDays, width: 12, height: 12, fill: "currentColor" }
                                                        p { "{post.created_at}" }
                                                    }
                                                    div { class: "inline-flex gap-1.5 items-center px-2.5 py-1 rounded-full border bg-white/5 border-white/10",
                                                        Icon { icon: FaUser, width: 12, height: 12, fill: "currentColor" }
                                                        p { "{post.author.name}" }
                                                    }
                                                }

                                                div { class: "inline-flex gap-2 items-center text-sm font-semibold text-[#ffef5c]",
                                                    span { "Read article" }
                                                    span { class: "transition-transform duration-300 group-hover:translate-x-1", "→" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Err(err) => rsx! {
                            div { class: "text-red-300", "Failed to load posts: {err}" }
                        },
                    }
                }
            }
        }
    }
}
