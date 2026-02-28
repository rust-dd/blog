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
            div { class: "container max-w-4xl mx-auto",
                if let Some(result) = posts.read().as_ref() {
                    match result {
                        Ok(items) => rsx! {
                            div { class: "flex flex-col gap-6",
                                for post in items {
                                    article { class: "flex flex-col p-3 text-left text-white rounded-lg transition-all duration-500 cursor-pointer break-inside-avoid bg-[#2a2a2a] hover:text-[#ffef5c]",
                                        Link {
                                            to: Route::Post { slug: post.slug.clone().unwrap_or_default() },
                                            class: "no-underline",
                                            div { class: "flex flex-col gap-1 mb-4 font-medium",
                                                p { class: "text-lg font-semibold line-clamp-2", "{post.title}" }
                                                p { class: "font-light text-xs text-gray-300", "{post.summary}" }
                                            }
                                            div { class: "flex flex-wrap gap-3 justify-start items-center text-xxs text-gray-300",
                                                div { class: "flex flex-row gap-1 items-center",
                                                    Icon { icon: FaClock, width: 12, height: 12, fill: "currentColor" }
                                                    p { "{post.read_time} min read" }
                                                }
                                                div { class: "flex flex-row gap-1 items-center",
                                                    Icon { icon: FaEye, width: 12, height: 12, fill: "currentColor" }
                                                    p { "{post.total_views} views" }
                                                }
                                                div { class: "flex flex-row gap-1 items-center",
                                                    Icon { icon: FaCalendarDays, width: 12, height: 12, fill: "currentColor" }
                                                    p { "{post.created_at}" }
                                                }
                                                div { class: "flex flex-row gap-1 items-center",
                                                    Icon { icon: FaUser, width: 12, height: 12, fill: "currentColor" }
                                                    p { "{post.author.name}" }
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
