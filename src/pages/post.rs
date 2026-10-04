use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;
use surrealdb_types::RecordIdKey;

use crate::{
    app::Route,
    authors,
    components::{
        post_list::PostList,
        shell::{SectionHeading, SideGroup, Sidebar},
    },
    seo,
    ssr::{
        api::{increment_views, select_post, select_related_posts},
        types::{Post, PostPage, RelatedPosts},
    },
};

#[component]
pub fn Component(slug: String) -> Element {
    // Each future must suspend right away: dioxus drops wakeups of a pending future the component
    // isn't suspended on yet, so starting both before `?` hangs SSR whenever the second one wins.
    let page = use_server_future({
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
            if let Some(Ok(Some(page))) = page.read().as_ref() {
                view_counted.set(true);
                if let RecordIdKey::String(id) = page.post.id.key.clone() {
                    spawn(async move {
                        let _ = increment_views(id).await;
                    });
                }
            }
        }
    });

    match page.read().as_ref() {
        Some(Ok(None)) => FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None),
        Some(Err(err)) => {
            FullstackContext::commit_error_status(err.clone());
        }
        _ => {}
    }

    let related = related
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    rsx! {
        match page.read().as_ref() {
            Some(Ok(Some(page))) => rsx! {
                Article { page: page.clone(), related }
            },
            Some(Ok(None)) => rsx! {
                seo::PageMeta {
                    title: seo::page_title("Post not found"),
                    description: "This post doesn't exist or has moved. Browse every Rust-DD post from the home page.",
                    path: format!("/post/{slug}"),
                    noindex: true,
                }
                Missing { heading: "Post not found", detail: format!("No post lives at /post/{slug}.") }
            },
            Some(Err(err)) => rsx! {
                seo::PageMeta {
                    title: seo::page_title("Failed to load post"),
                    description: "This post could not be loaded right now. Please try again shortly.",
                    path: format!("/post/{slug}"),
                    noindex: true,
                }
                Missing { heading: "Failed to load post", detail: err.to_string() }
            },
            None => rsx! {},
        }
    }
}

#[component]
fn Missing(heading: &'static str, detail: String) -> Element {
    rsx! {
        Sidebar {}
        main { id: "main", class: "shell-main",
            div { class: "doc-title", style: "margin-top: 36px",
                h1 { class: "doc-h1", "{heading}" }
            }
            p { class: "doc-lead", "{detail}" }
            p { class: "doc-text", style: "margin-top: 18px",
                Link { to: Route::Home { q: String::new() }, class: "text-mod", "Back to all posts" }
            }
        }
    }
}

#[component]
fn Article(page: PostPage, related: RelatedPosts) -> Element {
    let post = page.post.clone();
    let slug = post.slug.clone().unwrap_or_default();
    let path = format!("/post/{slug}");
    let url = seo::absolute_url(&path);
    let share_image = post
        .header_image
        .clone()
        .unwrap_or_else(|| seo::DEFAULT_OG_IMAGE.to_string());
    let topic = post.topic.clone();
    let note = authors::find(&post.author.name);
    let is_daniel = note.is_some_and(|note| note.href.is_none());
    let ident = note
        .map(|note| note.ident.to_string())
        .unwrap_or_else(|| post.author.name.clone());
    let avatar = post
        .author
        .github
        .as_ref()
        .map(|github| format!("{}.png?size=104", github.trim_end_matches('/')));
    let author_profile = [&post.author.github, &post.author.linkedin, &post.author.twitter]
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
        div { class: "reading-progress" }

        Sidebar {
            if !page.sections.is_empty() {
                SideGroup { title: "Sections",
                    for section in page.sections.iter() {
                        li { a { href: "#{section.id}", class: "side-link", "{section.title}" } }
                    }
                }
            }
            if let Some(topic) = topic.clone() {
                SideGroup { title: format!("In rust_dd::{topic}"),
                    li {
                        a {
                            href: "#main",
                            aria_current: "page",
                            class: "side-link side-current text-post",
                            "{post.title}"
                        }
                    }
                    for sibling in related.same_topic.iter() {
                        li {
                            Link {
                                to: Route::Post { slug: sibling.slug.clone().unwrap_or_default() },
                                class: "side-link text-post",
                                "{sibling.title}"
                            }
                        }
                    }
                }
            }
        }

        main { id: "main", class: "shell-main",
            article {
                p { class: "doc-path",
                    Link { to: Route::Home { q: String::new() }, class: "text-mod", "rust_dd" }
                    if let Some(topic) = topic.clone() {
                        "::"
                        Link { to: Route::Home { q: topic.clone() }, class: "text-mod", "{topic}" }
                    }
                }
                div { class: "doc-title",
                    h1 { class: "doc-h1 doc-h1-post", "{post.title}" }
                    time { class: "doc-source", datetime: "{post.created_at}", "{post.published_on()}" }
                }
                FrontMatter { post: post.clone() }
                p { class: "doc-lead", style: "font-size: 20px; color: rgb(var(--heading))", "{post.summary}" }
                if let Some(image) = post.header_image.clone() {
                    img {
                        src: "{image}",
                        alt: "{post.title}",
                        class: "mt-6 max-h-[520px] w-full rounded-md object-cover",
                    }
                }
                div {
                    class: "prose post-body",
                    style: "margin-top: 26px",
                    dangerous_inner_html: "{post.body}",
                }

                if post.show_cta {
                    aside { class: "doc-note",
                        p { class: "text-heading font-semibold", "Need Rust expertise?" }
                        p { class: "doc-text", "Build your next production Rust system with us." }
                        a {
                            href: "mailto:info@rust-dd.com",
                            class: "button-primary",
                            style: "margin-top: 12px",
                            "Contact us"
                        }
                    }
                }

                if !related.see_also.is_empty() {
                    section { aria_labelledby: "post-see-also",
                        SectionHeading { id: "post-see-also", title: "See also" }
                        PostList { posts: related.see_also.clone() }
                    }
                }

                section { aria_labelledby: "post-author",
                    SectionHeading { id: "post-author", title: "Author" }
                    div { class: "author-card",
                        if let Some(src) = avatar {
                            img { src: "{src}", alt: "", width: "52", height: "52", class: "author-avatar" }
                        }
                        div {
                            if is_daniel {
                                Link { to: Route::Home { q: String::new() }, class: "ident text-author", "{ident}" }
                            } else if let Some(profile) = author_profile {
                                a {
                                    href: "{profile}",
                                    rel: "author noopener noreferrer",
                                    target: "_blank",
                                    class: "ident text-author",
                                    "{ident}"
                                }
                            } else {
                                span { class: "ident text-author", "{ident}" }
                            }
                            if let Some(note) = note {
                                p { class: "doc-text", style: "margin-top: 4px; font-size: 17px", "{note.note}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FrontMatter(post: Post) -> Element {
    let published = post.created_at.get(..10).unwrap_or(&post.created_at).to_string();
    let updated = post
        .updated_on()
        .and(post.content_updated_at.as_deref())
        .map(|updated| updated.get(..10).unwrap_or(updated).to_string());

    rsx! {
        pre { class: "code-block front-matter", aria_label: "Post metadata",
            code {
                span { class: "fm-line", span { class: "sx-keyword", "[post]" } }
                span { class: "fm-line",
                    "author = "
                    span { class: "sx-string", "\"{post.author.name}\"" }
                }
                span { class: "fm-line",
                    "published = "
                    span { class: "sx-constant", "{published}" }
                }
                if let Some(updated) = updated {
                    span { class: "fm-line",
                        "updated = "
                        span { class: "sx-constant", "{updated}" }
                    }
                }
                span { class: "fm-line",
                    "reading-time = "
                    span { class: "sx-string", "\"{post.read_time} min\"" }
                }
                if let Some(topic) = post.topic.clone() {
                    span { class: "fm-line",
                        "topic = "
                        span { class: "sx-string", "\"{topic}\"" }
                    }
                }
                if !post.tags.is_empty() {
                    span { class: "fm-line",
                        "tags = ["
                        for (index, tag) in post.tags.iter().enumerate() {
                            if index > 0 {
                                ", "
                            }
                            span { class: "sx-string", "\"{tag}\"" }
                        }
                        "]"
                    }
                }
            }
        }
    }
}
