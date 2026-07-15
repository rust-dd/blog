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
                let id = surrealdb_types::ToSql::to_sql(&post.id.key);
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
                                div { class: "reading-progress" }

                                Link {
                                    to: Route::Home {},
                                    class: "inline-flex gap-1 text-xs text-faint transition-colors duration-200 hover:text-accent",
                                    span { "<-" }
                                    span { "back" }
                                }

                                article { class: "mt-4",
                                    section { class: "rounded-lg border border-border bg-surface p-5 sm:p-7 md:p-10",
                                        p { class: "text-xs text-faint", "// article" }
                                        h1 { class: "mt-2 text-2xl font-semibold leading-tight text-fg sm:text-3xl md:text-4xl", "{post.title}" }
                                        p { class: "mt-3 text-sm leading-relaxed text-muted", "{post.summary}" }

                                        p { class: "mt-4 text-xs text-faint",
                                            "author={post.author.name} date={post.created_at} read={post.read_time}min views={post.total_views}"
                                        }

                                        if !post.tags.is_empty() {
                                            {
                                                let tags_str = post.tags.iter().take(10).cloned().collect::<Vec<_>>().join(", ");
                                                rsx! {
                                                    p { class: "mt-2 text-xs text-faint",
                                                        span { class: "text-faint", "use " }
                                                        span { class: "text-muted", "tags" }
                                                        span { class: "text-faint", "::" }
                                                        span { class: "text-faint", "{{" }
                                                        span { class: "text-fg", "{tags_str}" }
                                                        span { class: "text-faint", "}};" }
                                                    }
                                                }
                                            }
                                        }

                                        if let Some(image) = post.header_image.clone() {
                                            div { class: "mt-6 overflow-hidden rounded-lg border border-border bg-surface-2",
                                                img {
                                                    src: "{image}",
                                                    alt: "{post.title}",
                                                    class: "max-h-[520px] w-full object-cover"
                                                }
                                            }
                                        }
                                    }

                                    div { class: "mt-4 rounded-lg border border-border bg-surface p-4 sm:p-6 md:p-8",
                                        div {
                                            class: "prose prose-base sm:prose-lg max-w-none break-words font-sans prose-pre:rounded-lg prose-pre:px-4 prose-pre:py-3 prose-pre:overflow-x-auto prose-pre:whitespace-pre prose-pre:max-w-full prose-code:bg-surface-2 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-code:font-normal prose-code:before:content-none prose-code:after:content-none prose-code:break-words prose-a:break-all prose-p:leading-7 sm:prose-p:leading-8 prose-li:leading-7 [&_img]:h-auto [&_img]:max-w-full [&_table]:block [&_table]:max-w-full [&_table]:overflow-x-auto [&_table]:text-sm",
                                            dangerous_inner_html: "{post.body}"
                                        }
                                    }

                                    if post.show_cta {
                                        div { class: "mt-4 rounded-lg border border-dashed border-border bg-surface p-4 sm:p-5",
                                            div { class: "flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between",
                                                div {
                                                    p { class: "text-sm font-semibold text-fg", "Need Rust expertise?" }
                                                    p { class: "text-xs text-muted", "Build your next production Rust system with us." }
                                                }
                                                a {
                                                    href: "mailto:info@rust-dd.com",
                                                    class: "inline-flex items-center justify-center rounded bg-accent px-4 py-2 text-xs font-semibold text-accent-fg transition-colors duration-200 hover:bg-accent/90",
                                                    "contact us"
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
                            p { class: "mt-4 text-muted", "{err}" }
                            Link {
                                to: Route::Home {},
                                class: "inline-flex mt-8 text-accent hover:underline",
                                "Go back home"
                            }
                        }
                    },
                }
            }
        }
    }
}
