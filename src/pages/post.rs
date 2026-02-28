use dioxus::prelude::*;

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

                            div { class: "w-full font-mono",
                                Link {
                                    to: Route::Home {},
                                    class: "inline-flex gap-1 text-xs text-slate-400 transition-colors duration-200 hover:text-slate-600",
                                    span { "<-" }
                                    span { "back" }
                                }

                                article { class: "mt-4",
                                    // Header
                                    section { class: "rounded-lg border border-slate-200 bg-white p-5 sm:p-7 md:p-10",
                                        p { class: "text-xs text-slate-400", "// article" }
                                        h1 { class: "mt-2 text-2xl font-semibold leading-tight text-slate-900 sm:text-3xl md:text-4xl", "{post.title}" }
                                        p { class: "mt-3 text-sm leading-relaxed text-slate-600", "{post.summary}" }

                                        // Metadata inline
                                        p { class: "mt-4 text-xs text-slate-400",
                                            "author={post.author.name} date={post.created_at} read={post.read_time}min views={post.total_views}"
                                        }

                                        // Tags
                                        if !post.tags.is_empty() {
                                            {
                                                let tags_str = post.tags.iter().take(10).cloned().collect::<Vec<_>>().join(", ");
                                                rsx! {
                                                    p { class: "mt-2 text-xs text-slate-400",
                                                        span { class: "text-slate-300", "use " }
                                                        span { class: "text-slate-400", "tags" }
                                                        span { class: "text-slate-300", "::" }
                                                        span { class: "text-slate-300", "{{" }
                                                        span { class: "text-slate-500", "{tags_str}" }
                                                        span { class: "text-slate-300", "}};" }
                                                    }
                                                }
                                            }
                                        }

                                        if let Some(image) = post.header_image.clone() {
                                            div { class: "mt-6 overflow-hidden rounded-lg border border-slate-200 bg-slate-100",
                                                img {
                                                    src: "{image}",
                                                    alt: "{post.title}",
                                                    class: "object-cover w-full max-h-[520px]"
                                                }
                                            }
                                        }
                                    }

                                    // Body
                                    div { class: "mt-4 rounded-lg border border-slate-200 bg-white p-4 sm:p-6 md:p-8",
                                        div {
                                            class: "prose prose-base sm:prose-lg max-w-none break-words prose-h3:text-slate-900 prose-h4:text-slate-900 prose-code:before:content-none prose-th:text-slate-900 prose-li:marker:text-slate-500 prose-code:after:content-none prose-pre:bg-slate-100 prose-pre:rounded-lg prose-pre:px-4 prose-pre:py-3 prose-code:text-slate-700 prose-code:bg-slate-100 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-strong:text-slate-900 prose-table:text-slate-800 prose-thead:text-slate-900 prose-li:text-slate-700 prose-ol:text-slate-700 prose-h1:text-slate-900 prose-h1:text-3xl prose-h2:text-slate-900 prose-h2:text-2xl prose-ul:text-slate-700 prose-p:text-slate-700 prose-a:text-slate-700 prose-p:leading-7 sm:prose-p:leading-8 prose-li:leading-7 prose-pre:whitespace-pre prose-pre:overflow-x-auto prose-pre:max-w-full prose-code:break-words prose-a:break-all [&_img]:h-auto [&_img]:max-w-full [&_table]:block [&_table]:max-w-full [&_table]:overflow-x-auto [&_table]:text-sm",
                                            dangerous_inner_html: "{post.body}"
                                        }
                                    }

                                    // CTA
                                    if post.show_cta {
                                        div { class: "mt-4 rounded-lg border border-dashed border-slate-300 bg-white p-4 sm:p-5",
                                            div { class: "flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between",
                                                div {
                                                    p { class: "text-sm font-semibold text-slate-700", "Need Rust expertise?" }
                                                    p { class: "text-xs text-slate-500", "Build your next production Rust system with us." }
                                                }
                                                Link {
                                                    to: Route::HireUs {},
                                                    class: "inline-flex items-center justify-center rounded bg-slate-700 px-4 py-2 text-xs font-semibold text-white transition-colors duration-200 hover:bg-slate-800",
                                                    "hire us"
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
