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

                            div { class: "flex flex-col w-full items-center mx-auto max-w-3xl",
                                if let Some(image) = post.header_image.clone() {
                                    img {
                                        src: "{image}",
                                        alt: "",
                                        class: "object-contain self-center mb-6 h-full rounded-lg w-fit aspect-auto"
                                    }
                                }
                                article { class: "w-full",
                                    div { class: "flex flex-col gap-4 mx-auto max-w-3xl",
                                        p { class: "text-4xl font-semibold", "{post.title}" }
                                        div { class: "flex flex-wrap gap-3 justify-start items-center text-sm text-gray-300",
                                            if let Some(github) = post.author.github.clone() {
                                                div { class: "flex flex-row gap-1 items-center",
                                                    Icon { icon: FaUser, width: 14, height: 14, fill: "currentColor" }
                                                    a {
                                                        href: "{github}",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        class: "cursor-pointer hover:underline",
                                                        "by "
                                                        span { class: "ml-1 font-semibold", "{post.author.name}" }
                                                    }
                                                }
                                            } else {
                                                div { class: "flex flex-row gap-1 items-center",
                                                    Icon { icon: FaUser, width: 14, height: 14, fill: "currentColor" }
                                                    p { "by " span { class: "ml-1 font-semibold", "{post.author.name}" } }
                                                }
                                            }
                                            div { class: "flex flex-row gap-1 items-center",
                                                Icon { icon: FaCalendarDays, width: 14, height: 14, fill: "currentColor" }
                                                p { "{post.created_at}" }
                                            }
                                            div { class: "flex flex-row gap-1 items-center",
                                                Icon { icon: FaClock, width: 14, height: 14, fill: "currentColor" }
                                                p { "{post.read_time} min read" }
                                            }
                                            div { class: "flex flex-row gap-1 items-center",
                                                Icon { icon: FaEye, width: 14, height: 14, fill: "currentColor" }
                                                p { "{post.total_views} views" }
                                            }
                                        }
                                    }
                                    div {
                                        class: "my-6 mx-auto max-w-3xl prose prose-h3:text-white prose-h4:text-white prose-code:before:content-none prose-th:text-white prose-li:marker:text-white prose-code:after:content-none prose-pre:bg-transparent prose-pre:rounded-lg prose-pre:p-0 prose-code:text-[#ffef5c] prose-strong:text-white prose-table:text-white prose-thead:text-white prose-li:text-white prose-ol:text-white prose-h1:text-white prose-h1:text-3xl prose-h2:text-white prose-h2:text-2xl prose-ul:text-white prose-p:text-white prose-a:text-[#ffef5c] prose-pre:whitespace-pre-wrap prose-pre:break-all prose-pre:overflow-auto",
                                        dangerous_inner_html: "{post.body}"
                                    }
                                }
                                if post.show_cta {
                                    div { class: "my-10",
                                        div { class: "flex flex-col md:flex-row items-center justify-between bg-[#2E2E2E] rounded-lg p-8",
                                            div { class: "mb-6 md:mb-0 md:mr-8 text-center md:text-left",
                                                p { class: "text-2xl md:text-3xl font-bold text-[#ffef5c] mb-2", "Need Rust Expertise?" }
                                                p { class: "text-gray-300 max-w-md", "Our team of Rust developers is ready to bring your high-performance projects to life." }
                                            }
                                            Link {
                                                to: Route::HireUs {},
                                                class: "bg-[#ffef5c] text-gray-900 px-6 py-3 text-lg font-semibold rounded-full transition-colors duration-300",
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
