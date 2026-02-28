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
                                    class: "inline-flex gap-2 items-center text-sm text-slate-600 transition-colors duration-200 hover:text-slate-700",
                                    span { "←" }
                                    span { "Back to blog" }
                                }

                                div { class: "grid items-start gap-6 mt-4 md:gap-8 lg:grid-cols-[minmax(0,1fr)_280px]",
                                    article { class: "min-w-0",
                                        section { class: "relative overflow-hidden p-5 rounded-3xl border border-slate-200 bg-white/92 shadow-sm sm:p-7 md:p-10",
                                            div { class: "absolute -top-20 right-0 w-72 h-72 rounded-full bg-slate-300/20 blur-3xl pointer-events-none" }
                                            p { class: "relative z-10 text-[11px] font-semibold tracking-[0.2em] text-slate-700 uppercase", "Article" }
                                            h1 { class: "relative z-10 mt-3 max-w-4xl text-2xl font-semibold leading-tight text-slate-900 sm:text-4xl md:text-5xl", "{post.title}" }
                                            p { class: "relative z-10 mt-3 max-w-3xl text-sm leading-relaxed text-slate-600 md:text-base", "{post.summary}" }

                                            div { class: "relative z-10 flex flex-wrap gap-2 mt-4 text-xs text-slate-600",
                                                div { class: "inline-flex items-center gap-2 rounded-full border border-slate-200 bg-slate-50 px-2.5 py-1.5",
                                                    Icon { icon: FaUser, width: 12, height: 12, fill: "currentColor" }
                                                    span { "{post.author.name}" }
                                                }
                                                div { class: "inline-flex items-center gap-2 rounded-full border border-slate-200 bg-slate-50 px-2.5 py-1.5",
                                                    Icon { icon: FaCalendarDays, width: 12, height: 12, fill: "currentColor" }
                                                    span { "{post.created_at}" }
                                                }
                                                div { class: "inline-flex items-center gap-2 rounded-full border border-slate-200 bg-slate-50 px-2.5 py-1.5",
                                                    Icon { icon: FaClock, width: 12, height: 12, fill: "currentColor" }
                                                    span { "{post.read_time} min read" }
                                                }
                                                div { class: "hidden sm:inline-flex items-center gap-2 rounded-full border border-slate-200 bg-slate-50 px-2.5 py-1.5",
                                                    Icon { icon: FaEye, width: 12, height: 12, fill: "currentColor" }
                                                    span { "{post.total_views} views" }
                                                }
                                            }

                                            if let Some(image) = post.header_image.clone() {
                                                div { class: "overflow-hidden relative z-10 mt-6 rounded-3xl border border-slate-200 bg-slate-100",
                                                    img {
                                                        src: "{image}",
                                                        alt: "{post.title}",
                                                        class: "object-cover w-full max-h-[520px]"
                                                    }
                                                }
                                            }
                                        }

                                        div { class: "overflow-hidden p-4 mt-6 rounded-3xl border border-slate-200 bg-white/95 shadow-sm sm:p-6 md:p-8",
                                            div {
                                                class: "prose prose-base sm:prose-lg max-w-none break-words prose-h3:text-slate-900 prose-h4:text-slate-900 prose-code:before:content-none prose-th:text-slate-900 prose-li:marker:text-slate-500 prose-code:after:content-none prose-pre:bg-slate-100 prose-pre:rounded-xl prose-pre:px-4 prose-pre:py-3 prose-code:text-slate-700 prose-code:bg-slate-100 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-strong:text-slate-900 prose-table:text-slate-800 prose-thead:text-slate-900 prose-li:text-slate-700 prose-ol:text-slate-700 prose-h1:text-slate-900 prose-h1:text-3xl prose-h2:text-slate-900 prose-h2:text-2xl prose-ul:text-slate-700 prose-p:text-slate-700 prose-a:text-slate-700 prose-p:leading-7 sm:prose-p:leading-8 prose-li:leading-7 prose-pre:whitespace-pre prose-pre:overflow-x-auto prose-pre:max-w-full prose-code:break-words prose-a:break-all [&_img]:h-auto [&_img]:max-w-full [&_table]:block [&_table]:max-w-full [&_table]:overflow-x-auto [&_table]:text-sm",
                                                dangerous_inner_html: "{post.body}"
                                            }
                                        }
                                    }

                                    aside { class: "min-w-0 space-y-4 h-fit lg:sticky lg:top-24",
                                        div { class: "overflow-hidden relative p-4 rounded-2xl border border-slate-200 bg-white/90 shadow-sm",
                                            p { class: "text-[11px] font-semibold tracking-[0.18em] text-slate-500 uppercase", "meta" }
                                            div { class: "mt-3 space-y-2 text-xs text-slate-600",
                                                p { span { class: "text-slate-700", "$ " } "author = " span { class: "text-slate-900", "{post.author.name}" } }
                                                p { span { class: "text-slate-700", "$ " } "published = " span { class: "text-slate-900", "{post.created_at}" } }
                                                p { span { class: "text-slate-700", "$ " } "read = " span { class: "text-slate-900", "{post.read_time} min" } }
                                                p { span { class: "text-slate-700", "$ " } "views = " span { class: "text-slate-900", "{post.total_views}" } }
                                            }
                                        }

                                        if !post.tags.is_empty() {
                                            div { class: "p-4 rounded-2xl border border-slate-200 bg-white/90 shadow-sm",
                                                p { class: "text-[11px] font-semibold tracking-[0.18em] text-slate-500 uppercase", "tags" }
                                                div { class: "flex flex-wrap gap-2 mt-3",
                                                    for tag in post.tags.iter().take(10) {
                                                        span { class: "rounded-full border border-slate-200 bg-slate-50 px-2.5 py-1 text-[11px] text-slate-700", "#{tag}" }
                                                    }
                                                }
                                            }
                                        }

                                        if post.show_cta {
                                            div { class: "p-4 rounded-2xl border border-slate-300 bg-slate-50/80 shadow-sm",
                                                p { class: "text-lg font-semibold text-slate-700", "Need Rust Expertise?" }
                                                p { class: "mt-2 text-sm text-slate-600", "Build your next production Rust system with us." }
                                                Link {
                                                    to: Route::HireUs {},
                                                    class: "inline-flex mt-4 items-center justify-center rounded-full bg-slate-600 px-4 py-2 text-sm font-semibold text-white transition-all duration-200 hover:bg-slate-700",
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
                            h1 { class: "text-3xl font-semibold text-red-500", "Failed to load post" }
                            p { class: "mt-4 text-slate-600", "{err}" }
                            Link {
                                to: Route::Home {},
                                class: "inline-flex mt-8 text-slate-600 hover:underline",
                                "Go back home"
                            }
                        }
                    },
                }
            }
        }
    }
}
