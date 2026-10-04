use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;

use crate::{
    app::Route,
    authors::AUTHORS,
    components::{
        post_list::PostList,
        shell::{SectionHeading, SideGroup, Sidebar},
    },
    search, seo,
    ssr::{
        api::{select_latest_snippet, select_posts},
        types::Post,
    },
    topics::TOPICS,
};

const FIRST_PAGE: usize = 12;

#[component]
pub fn Component(query: String) -> Element {
    let posts = use_server_future(select_posts)?;
    let snippet = use_server_future(select_latest_snippet)?;

    // A 5xx keeps crawlers from indexing the error state as the home page.
    if let Some(Err(err)) = posts.read().as_ref() {
        FullstackContext::commit_error_status(err.clone());
    }
    let load_error = posts
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().err())
        .map(|err| err.to_string());
    let all: Vec<Post> = posts
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .cloned()
        .unwrap_or_default();
    let snippet = snippet
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .cloned()
        .flatten();

    let searching = !query.trim().is_empty();
    let matching: Vec<Post> = all
        .iter()
        .filter(|post| search::matches(&post.title, post.topic.as_deref(), &query))
        .cloned()
        .collect();
    let first_page: Vec<Post> = all.iter().take(FIRST_PAGE).cloned().collect();
    let rest: Vec<Post> = all.iter().skip(FIRST_PAGE).cloned().collect();
    let description = seo::SITE_DESCRIPTION;

    rsx! {
        seo::PageMeta {
            title: seo::HOME_TITLE,
            description: seo::SITE_DESCRIPTION,
            path: "/",
        }
        seo::JsonLd { value: seo::website_graph() }

        Sidebar { meta: format!("{} posts", all.len()),
            SideGroup { title: "Sections",
                li { a { href: "#latest", class: "side-link", "Latest" } }
                li { a { href: "#modules", class: "side-link", "Modules" } }
                li { a { href: "#posts", class: "side-link", "Posts" } }
                li { a { href: "#authors", class: "side-link", "Authors" } }
            }
            SideGroup { title: "Modules",
                for topic in TOPICS.iter() {
                    li {
                        Link {
                            to: Route::Home { q: topic.name.to_string() },
                            class: "side-link side-code text-mod",
                            "{topic.name}"
                        }
                    }
                }
            }
            SideGroup { title: "Authors",
                for author in AUTHORS.iter() {
                    li {
                        AuthorLink {
                            ident: author.ident,
                            href: author.href,
                            class: "side-link side-code text-author",
                        }
                    }
                }
            }
        }

        main { id: "main", class: "shell-main",
            div { class: "doc-title", style: "margin-top: 36px",
                h1 { class: "doc-h1",
                    "Crate "
                    span { class: "text-mod", "rust_dd" }
                }
                a {
                    href: "https://github.com/rust-dd/blog",
                    rel: "noopener noreferrer",
                    target: "_blank",
                    class: "doc-source",
                    "Source"
                }
            }
            p { class: "doc-lead",
                "{description} Written by "
                AuthorLink { ident: "DanielBoros", href: None, class: "ident text-author" }
                " and "
                AuthorLink {
                    ident: "DanielZelei",
                    href: Some("https://github.com/zeldan"),
                    class: "ident text-author",
                }
                "."
            }

            if let Some(latest) = all.first().cloned() {
                section { aria_labelledby: "latest",
                    SectionHeading { id: "latest", title: "Latest" }
                    article { class: "latest",
                        p { class: "item-meta",
                            time { datetime: "{latest.created_at}", "{latest.published_on()}" }
                            span { "{latest.read_time} min read" }
                            if let Some(topic) = latest.topic.clone() {
                                span {
                                    "in "
                                    Link { to: Route::Home { q: topic.clone() }, class: "ident text-mod", "{topic}" }
                                }
                            }
                        }
                        h3 { class: "latest-title",
                            Link { to: Route::Post { slug: latest.slug.clone().unwrap_or_default() }, "{latest.title}" }
                        }
                        p { class: "doc-text", style: "margin-top: 10px; max-width: 40em", "{latest.summary}" }
                        if let Some(html) = snippet {
                            div { class: "latest-code", dangerous_inner_html: "{html}" }
                        }
                    }
                }
            }

            section { aria_labelledby: "modules",
                SectionHeading { id: "modules", title: "Modules" }
                dl { class: "item-table",
                    for topic in TOPICS.iter() {
                        div { class: "item-row",
                            dt {
                                Link {
                                    to: Route::Home { q: topic.name.to_string() },
                                    class: "ident text-mod",
                                    "{topic.name}"
                                }
                            }
                            dd { "{topic.description}" }
                        }
                    }
                }
            }

            section { aria_labelledby: "posts",
                SectionHeading { id: "posts", title: "Posts" }
                div { class: "list-status",
                    if searching {
                        p { "{matching.len()} of {all.len()} posts match “{query.trim()}”" }
                        Link { to: Route::Home { q: String::new() }, class: "button-quiet", "Clear filter" }
                    } else {
                        p { "{all.len()} posts, newest first" }
                    }
                }
                if let Some(err) = load_error {
                    p { class: "doc-text", "Failed to load posts: {err}" }
                }
                if searching {
                    PostList { posts: matching.clone() }
                    if matching.is_empty() {
                        p { class: "doc-text", style: "margin-top: 16px",
                            "No posts match that search. Try a module name such as quant or async."
                        }
                    }
                } else {
                    PostList { posts: first_page }
                    if !rest.is_empty() {
                        details { class: "more-posts",
                            summary { class: "button-quiet", "Show all {all.len()} posts" }
                            PostList { posts: rest }
                        }
                    }
                }
            }

            section { aria_labelledby: "authors",
                SectionHeading { id: "authors", title: "Authors" }
                dl { class: "item-table",
                    for author in AUTHORS.iter() {
                        div { class: "item-row",
                            dt { AuthorLink { ident: author.ident, href: author.href, class: "ident text-author" } }
                            dd { "{author.note}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AuthorLink(ident: &'static str, href: Option<&'static str>, class: &'static str) -> Element {
    rsx! {
        match href {
            Some(href) => rsx! {
                a { href, rel: "noopener noreferrer", target: "_blank", class, "{ident}" }
            },
            None => rsx! {
                Link { to: Route::About {}, class, "{ident}" }
            },
        }
    }
}
