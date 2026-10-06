use super::api::{published_posts, POST_FIELDS};
use super::markdown::process_markdown;
use super::types::Post;
use crate::seo::{absolute_url, meta_description, SITE_DESCRIPTION, SITE_NAME};
use axum::response::Response;
use chrono::{DateTime, Utc};
use dioxus::prelude::Result;
use rss::{ChannelBuilder, Item};
use surrealdb::engine::remote::http::Client;
use surrealdb::Surreal;

use crate::ssr::app_state::db;

pub async fn rss_handler() -> Response<String> {
    let db = db().await;
    let conn = db.get().await;
    let rss = match generate_rss(&conn).await {
        Ok(rss) => rss,
        // Serving an empty 200 would tell feed readers the blog has no posts;
        // a 503 keeps the last good copy in their cache until the db is back.
        Err(err) => {
            tracing::error!("rss generation failed: {err}");
            return Response::builder()
                .status(http::StatusCode::SERVICE_UNAVAILABLE)
                .header("Content-Type", "application/xml")
                .body(String::new())
                .unwrap();
        }
    };
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(rss)
        .unwrap()
}

pub async fn generate_rss(db: &Surreal<Client>) -> Result<String> {
    let mut query = db
        .query(format!(
            "SELECT {POST_FIELDS} FROM post WHERE is_published = true ORDER BY created_at DESC;"
        ))
        .await?;
    let mut posts = query.take::<Vec<Post>>(0)?;

    for post in &mut posts {
        let date_time = DateTime::parse_from_rfc3339(&post.created_at)
            .unwrap()
            .with_timezone(&Utc);
        post.created_at = date_time.to_rfc2822();
        post.body = process_markdown(post.body.clone()).await?;
    }

    let channel = ChannelBuilder::default()
        .title("Rust-DD")
        .link("https://rust-dd.com")
        .description("Rust-DD Blog – Tech Insights & Consulting")
        .items(
            posts
                .into_iter()
                .map(|post| {
                    let mut item = Item::default();
                    item.set_author(post.author.name.to_string());
                    item.set_title(post.title.to_string());
                    item.set_description(post.body.to_string());
                    item.set_link(format!("https://rust-dd.com/post/{}", post.slug.unwrap_or_default()));
                    item.set_pub_date(post.created_at.to_string());
                    item
                })
                .collect::<Vec<_>>(),
        )
        .build();

    Ok(channel.to_string())
}

pub async fn sitemap_handler() -> Response<String> {
    use surrealdb_types::SurrealValue;

    #[derive(SurrealValue)]
    struct SitemapPost {
        slug: Option<String>,
        created_at: String,
        content_updated_at: Option<String>,
    }

    impl SitemapPost {
        fn lastmod(&self) -> &str {
            self.content_updated_at.as_deref().unwrap_or(&self.created_at)
        }
    }

    let db = db().await;
    let posts = match db
        .get()
        .await
        .query("SELECT slug, <string>created_at AS created_at, <option<string>>content_updated_at AS content_updated_at FROM post WHERE is_published = true ORDER BY created_at DESC;")
        .await
        .and_then(|mut query| query.take::<Vec<SitemapPost>>(0))
    {
        Ok(posts) => posts,
        // Unwrapping here panicked the worker thread on every crawl once the
        // db handle went stale; a 503 lets crawlers retry instead.
        Err(err) => {
            tracing::error!("sitemap generation failed: {err}");
            return Response::builder()
                .status(http::StatusCode::SERVICE_UNAVAILABLE)
                .header("Content-Type", "application/xml")
                .body(String::new())
                .unwrap();
        }
    };
    let mut sitemap = String::new();
    sitemap.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    sitemap.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    // Only indexable HTML pages belong here. No changefreq/priority: Google ignores both. lastmod
    // uses content_updated_at, not updated_at, which changes on every view count.
    let latest_change = posts.iter().map(SitemapPost::lastmod).max();
    for (path, lastmod) in [
        ("/", latest_change),
        ("/about", None),
        ("/projects", None),
        ("/opensource", None),
    ] {
        push_sitemap_url(&mut sitemap, &absolute_url(path), lastmod);
    }
    for post in &posts {
        if let Some(slug) = &post.slug {
            push_sitemap_url(
                &mut sitemap,
                &absolute_url(&format!("/post/{slug}")),
                Some(post.lastmod()),
            );
        }
    }
    sitemap.push_str("</urlset>");
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(sitemap)
        .unwrap()
}

fn push_sitemap_url(sitemap: &mut String, loc: &str, lastmod: Option<&str>) {
    sitemap.push_str("<url>\n");
    sitemap.push_str(&format!("<loc>{loc}</loc>\n"));
    if let Some(lastmod) = lastmod {
        sitemap.push_str(&format!("<lastmod>{lastmod}</lastmod>\n"));
    }
    sitemap.push_str("</url>\n");
}

pub async fn robots_handler() -> Response<String> {
    let robots = format!(
        "User-agent: *\nAllow: /\nDisallow: /admin\nDisallow: /api/admin/\n\nSitemap: {}\n",
        absolute_url("/sitemap.xml")
    );
    Response::builder()
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(robots)
        .unwrap()
}

/// Plain-markdown site index for LLM crawlers, per https://llmstxt.org.
pub async fn llms_txt_handler() -> Response<String> {
    let posts = match published_posts().await {
        Ok(posts) => posts,
        Err(err) => {
            tracing::error!("llms.txt generation failed: {err}");
            return Response::builder()
                .status(http::StatusCode::SERVICE_UNAVAILABLE)
                .header("Content-Type", "text/plain; charset=utf-8")
                .body(String::new())
                .unwrap();
        }
    };

    let mut llms = format!("# {SITE_NAME}\n\n> {SITE_DESCRIPTION}\n\n## Posts\n\n");
    for post in &posts {
        if let Some(slug) = &post.slug {
            let url = absolute_url(&format!("/post/{slug}"));
            llms.push_str(&format!(
                "- [{}]({url}): {}\n",
                post.title,
                meta_description(&post.summary)
            ));
        }
    }
    llms.push_str("\n## Pages\n\n");
    llms.push_str(&format!(
        "- [About Daniel Boros]({}): Bio, CV, research and open-source work of the main author\n",
        absolute_url("/about")
    ));
    llms.push_str(&format!(
        "- [Projects]({}): Apps and developer tools built with Rust\n",
        absolute_url("/projects")
    ));
    llms.push_str(&format!(
        "- [Open source]({}): Open-source Rust crates, frameworks and CLI tools\n",
        absolute_url("/opensource")
    ));
    llms.push_str(&format!(
        "- [RSS feed]({}): Full text of every post\n",
        absolute_url("/rss.xml")
    ));

    Response::builder()
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(llms)
        .unwrap()
}
