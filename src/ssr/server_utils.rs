use super::types::Post;
use axum::response::Response;
use chrono::{DateTime, Utc};
use dioxus::prelude::Result;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, TextMergeStream};
use regex::Regex;
use rss::{ChannelBuilder, Item};
use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::engine::remote::http::{Client, Http, Https};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use crate::ssr::app_state::db;

pub async fn connect() -> Surreal<Client> {
    let protocol = env::var("SURREAL_PROTOCOL").unwrap_or("http".to_string());
    let host = env::var("SURREAL_HOST").unwrap_or("127.0.0.1:8000".to_string());
    let username = env::var("SURREAL_ROOT_USER").unwrap_or("root".to_string());
    let password = env::var("SURREAL_ROOT_PASS").unwrap_or("root".to_string());
    let ns = env::var("SURREAL_NS").unwrap_or("rustblog".to_string());
    let db_name = env::var("SURREAL_DB").unwrap_or("rustblog".to_string());

    let db = if protocol == "http" {
        Surreal::new::<Http>(host).await.unwrap()
    } else {
        Surreal::new::<Https>(host).await.unwrap()
    };

    db.signin(Root {
        username: &username,
        password: &password,
    })
    .await
    .unwrap();
    db.use_ns(ns).use_db(db_name).await.unwrap();

    db
}

pub async fn rss_handler() -> Response<String> {
    let db = db().await;
    let rss = generate_rss(db).await.unwrap_or_default();
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(rss)
        .unwrap()
}

pub async fn generate_rss(db: Surreal<Client>) -> Result<String> {
    let mut query = db
        .query("SELECT *, author.* from post WHERE is_published = true ORDER BY created_at DESC;")
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

pub async fn process_markdown(markdown: String) -> Result<String> {
    struct MathEventProcessor {
        display_style_opts: katex::Opts,
    }

    impl MathEventProcessor {
        fn new() -> MathEventProcessor {
            let opts = katex::Opts::builder().display_mode(true).build().unwrap();
            MathEventProcessor {
                display_style_opts: opts,
            }
        }

        fn process_math_event<'a>(&'a self, event: Event<'a>) -> Event<'a> {
            match event {
                Event::InlineMath(math_exp) => Event::InlineHtml(CowStr::from(katex::render(&math_exp).unwrap())),
                Event::DisplayMath(math_exp) => Event::Html(CowStr::from(
                    katex::render_with_opts(&math_exp, &self.display_style_opts).unwrap(),
                )),
                _ => event,
            }
        }
    }

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = ts
        .themes
        .get("InspiredGitHub")
        .or_else(|| ts.themes.get("base16-ocean.light"))
        .or_else(|| ts.themes.values().next())
        .expect("syntect default theme missing");
    let re_img = Regex::new(r"!\[.*?\]\((.*?\.(svg|png|jpe?g|gif|bmp|webp))\)")?;
    let re_bg_styles = Regex::new(r"background-color:\s*#[0-9a-fA-F]{6};?")?;
    let re_empty_style = Regex::new(r#"style="\s*""#)?;

    let mut processed_markdown = String::new();
    let mut last_img_end = 0;
    for img_cap in re_img.captures_iter(&markdown) {
        processed_markdown.push_str(&markdown[last_img_end..img_cap.get(0).unwrap().start()]);
        let img_path = &img_cap[1];
        let img_format = &img_cap[2];
        let img_html = if img_format == "svg" {
            format!(
                r#"<div style="display: flex; justify-content: center;"><img src="{}" style="filter: invert(100%); width: 100%;"></div>"#,
                img_path
            )
        } else {
            format!(
                r#"<div style="display: flex; justify-content: center;"><img src="{}" style="width: 100%;"></div>"#,
                img_path
            )
        };
        processed_markdown.push_str(&img_html);
        last_img_end = img_cap.get(0).unwrap().end();
    }
    processed_markdown.push_str(&markdown[last_img_end..]);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_MATH);

    let parser = Parser::new_ext(&processed_markdown, options);
    let mep = MathEventProcessor::new();
    let iterator = TextMergeStream::new(parser).map(|event| mep.process_math_event(event));

    let mut events = Vec::new();
    let mut in_code_block = false;
    let mut code_block_language: Option<String> = None;
    let mut code_block_content = String::new();
    let mut skip_image = false;

    for event in iterator {
        if skip_image {
            if let Event::End(TagEnd::Image) = event {
                skip_image = false;
            }
            continue;
        }

        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_block_content.clear();
                code_block_language = match kind {
                    CodeBlockKind::Fenced(info) => {
                        Some(info.split_whitespace().next().unwrap_or("plaintext").to_string())
                    }
                    CodeBlockKind::Indented => Some("plaintext".to_string()),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let language = code_block_language.as_deref().unwrap_or("plaintext");
                let syntax = ps
                    .find_syntax_by_token(language)
                    .unwrap_or_else(|| ps.find_syntax_plain_text());
                let mut highlighted_html = highlighted_html_for_string(&code_block_content, &ps, syntax, theme)?;
                highlighted_html = re_bg_styles.replace_all(&highlighted_html, "").to_string();
                highlighted_html = re_empty_style.replace_all(&highlighted_html, "").to_string();
                events.push(Event::Html(CowStr::from(highlighted_html)));
                code_block_language = None;
            }
            Event::Text(text) if in_code_block => {
                code_block_content.push_str(&text);
            }
            Event::SoftBreak if in_code_block => {
                code_block_content.push('\n');
            }
            Event::HardBreak if in_code_block => {
                code_block_content.push('\n');
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                let img_path = dest_url.into_string();
                let img_format = img_path.split('.').last().unwrap_or("").to_lowercase();

                let img_html = if img_format == "svg" {
                    format!(
                        r#"<div style="display: flex; justify-content: center;"><img alt="image" src="{}" style="filter: invert(100%); width: 100%;"></div>"#,
                        img_path
                    )
                } else {
                    format!(
                        r#"<div style="display: flex; justify-content: center;"><img alt="image" src="{}" style="width: 100%;"></div>"#,
                        img_path
                    )
                };

                events.push(Event::Html(CowStr::from(img_html)));
                skip_image = true;
            }
            Event::End(TagEnd::Image) => {
                if !skip_image {
                    events.push(Event::End(TagEnd::Image));
                }
            }
            other if !in_code_block => events.push(other),
            _ => {}
        }
    }

    use pulldown_cmark::html::push_html;
    let mut html_output = String::new();
    push_html(&mut html_output, events.into_iter());

    Ok(html_output)
}

pub async fn sitemap_handler() -> Response<String> {
    #[derive(Serialize, Deserialize)]
    struct SitemapPost {
        slug: Option<String>,
        created_at: String,
    }

    let db = db().await;
    let query = db
        .query("SELECT slug, created_at FROM post WHERE is_published = true ORDER BY created_at DESC;")
        .await;
    let posts = query.unwrap().take::<Vec<SitemapPost>>(0).unwrap();
    let mut sitemap = String::new();
    sitemap.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    sitemap.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    let static_urls = vec![
        ("https://rust-dd.com/", "daily", "0.9"),
        ("https://rust-dd.com/hireus", "weekly", "1.0"),
        ("https://rust-dd.com/references", "weekly", "0.6"),
        ("https://rust-dd.com/rss.xml", "daily", "0.5"),
        ("https://rust-dd.com/sitemap.xml", "monthly", "0.5"),
    ];

    for (url, freq, priority) in static_urls {
        sitemap.push_str("<url>\n");
        sitemap.push_str(&format!("<loc>{}</loc>\n", url));
        sitemap.push_str(&format!("<changefreq>{}</changefreq>\n", freq));
        sitemap.push_str(&format!("<priority>{}</priority>\n", priority));
        sitemap.push_str("</url>\n");
    }

    for post in posts {
        if let Some(slug) = post.slug {
            sitemap.push_str("<url>\n");
            sitemap.push_str(&format!("<loc>https://rust-dd.com/post/{}</loc>\n", slug));
            sitemap.push_str("<changefreq>monthly</changefreq>\n");
            sitemap.push_str("<priority>1.0</priority>\n");
            sitemap.push_str(&format!("<lastmod>{}</lastmod>\n", post.created_at));
            sitemap.push_str("</url>\n");
        }
    }
    sitemap.push_str("</urlset>");
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(sitemap)
        .unwrap()
}

pub async fn robots_handler() -> Response<String> {
    let mut robots = String::new();
    robots.push_str("User-agent: *\nDisallow:\n\nAllow: /\n\nSitemap: https://rust-dd.com/sitemap.xml\n");
    Response::builder()
        .header("Content-Type", "text/plain")
        .body(robots)
        .unwrap()
}
