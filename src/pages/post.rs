use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;
use surrealdb_types::RecordIdKey;

use crate::{
    app::Route,
    seo,
    ssr::{
        api::{increment_views, select_post, select_related_posts},
        types::Post,
    },
};

#[component]
pub fn Component(slug: String) -> Element {
    // Each future must suspend right away: dioxus drops wakeups of a pending future the component
    // isn't suspended on yet, so starting both before `?` hangs SSR whenever the second one wins.
    let post = use_server_future({
        let slug = slug.clone();
        move || {
            let slug = slug.clone();
            async move { select_post(slug).await }
        }
    })?;
    let related = use_server_future({
        let slug = slug.clone();
        move || {
            let slug = slug.clone();
            async move { select_related_posts(slug).await }
        }
    })?;
    let mut view_counted = use_signal(|| false);

    use_effect(move || {
        if cfg!(not(debug_assertions)) && !*view_counted.read() {
            if let Some(Ok(Some(page))) = post.read().as_ref() {
                view_counted.set(true);
                if let RecordIdKey::String(id) = page.post.id.key.clone() {
                    spawn(async move {
                        let _ = increment_views(id).await;
                    });
                }
            }
        }
    });

    match post.read().as_ref() {
        Some(Ok(None)) => FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None),
        Some(Err(err)) => {
            FullstackContext::commit_error_status(err.clone());
        }
        _ => {}
    }

    let related_posts = related
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|related| related.see_also.clone())
        .unwrap_or_default();

    rsx! {
        crate::components::shell::Sidebar {}
        main { id: "main", class: "shell-main",
        match post.read().as_ref() {
            Some(Ok(Some(page))) => rsx! {
                Article { post: page.post.clone(), related: related_posts }
            },
            Some(Ok(None)) => rsx! {
                seo::PageMeta {
                    title: seo::page_title("Post not found"),
                    description: "This post doesn't exist or has moved. Browse every Rust-DD post from the home page.",
                    path: format!("/post/{slug}"),
                    noindex: true,
                }
                section { class: "mx-auto max-w-3xl text-center pt-24",
                    p { class: "text-xs text-faint", "// post not found" }
                    h1 { class: "mt-2 text-5xl font-bold text-accent", "404" }
                    p { class: "mt-4 text-lg text-muted", "Post not found: /post/{slug}" }
                    Link {
                        to: Route::Home { q: String::new() },
                        class: "inline-flex mt-8 text-accent hover:underline",
                        "Go back home"
                    }
                }
            },
            Some(Err(err)) => rsx! {
                seo::PageMeta {
                    title: seo::page_title("Failed to load post"),
                    description: "This post could not be loaded right now. Please try again shortly.",
                    path: format!("/post/{slug}"),
                    noindex: true,
                }
                section { class: "mx-auto max-w-3xl text-center pt-24",
                    h1 { class: "text-3xl font-semibold text-red-500", "Failed to load post" }
                    p { class: "mt-4 text-muted", "{err}" }
                    Link {
                        to: Route::Home { q: String::new() },
                        class: "inline-flex mt-8 text-accent hover:underline",
                        "Go back home"
                    }
                }
            },
            None => rsx! {},
        }
        }
    }
}

#[component]
fn Article(post: Post, related: Vec<Post>) -> Element {
    let path = format!("/post/{}", post.slug.clone().unwrap_or_default());
    let url = seo::absolute_url(&path);
    let share_image = post
        .header_image
        .clone()
        .unwrap_or_else(|| seo::DEFAULT_OG_IMAGE.to_string());
    let author_profile = [&post.author.linkedin, &post.author.github, &post.author.twitter]
        .into_iter()
        .flatten()
        .next()
        .cloned();

    rsx! {
        seo::PageMeta {
            title: seo::page_title(&post.title),
            description: seo::meta_description(&post.summary),
            path,
            og_type: "article",
            image: post.header_image.clone(),
            noindex: !post.is_published,
        }
        document::Meta { property: "article:published_time", content: "{post.created_at}" }
        if let Some(modified) = post.content_updated_at.clone() {
            document::Meta { property: "article:modified_time", content: "{modified}" }
        }
        seo::JsonLd { value: seo::blog_posting(&post, &url, &share_image) }

        div { class: "w-full font-mono",
            div { class: "reading-progress" }

            Link {
                to: Route::Home { q: String::new() },
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
                        "author="
                        if let Some(profile) = author_profile {
                            a {
                                href: "{profile}",
                                rel: "author noopener noreferrer",
                                target: "_blank",
                                class: "transition-colors duration-200 hover:text-accent",
                                "{post.author.name}"
                            }
                        } else {
                            "{post.author.name}"
                        }
                        " date="
                        time { datetime: "{post.created_at}", "{post.published_on()}" }
                        if let (Some(updated), Some(modified)) = (post.updated_on(), post.content_updated_at.clone()) {
                            " updated="
                            time { datetime: "{modified}", "{updated}" }
                        }
                        " read={post.read_time}min views={post.total_views}"
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

                div { class: "mt-4 rounded-lg border border-border bg-surface p-5 sm:p-7 md:p-10",
                    div {
                        class: "prose post-body",
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

            if !related.is_empty() {
                section { class: "mt-8",
                    h2 { class: "text-xs text-faint", "// related posts" }
                    div { class: "mt-3 flex flex-col gap-3",
                        for item in related.iter() {
                            Link {
                                to: Route::Post { slug: item.slug.clone().unwrap_or_default() },
                                class: "group block rounded-lg border border-border bg-surface p-4 no-underline transition-colors duration-200 hover:border-accent",
                                h3 { class: "text-sm leading-tight text-fg transition-colors duration-200 group-hover:text-accent sm:text-base",
                                    "{item.title}"
                                }
                                p { class: "mt-1 text-xs text-faint", "{item.published_on()} · {item.read_time}min" }
                            }
                        }
                    }
                }
            }
        }
    }
}
