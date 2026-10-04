use std::collections::HashSet;
use std::sync::OnceLock;

use super::api::{published_posts, POST_FIELDS};
use super::types::{Post, Section};
use crate::seo::{absolute_url, meta_description, SITE_DESCRIPTION, SITE_NAME};
use axum::response::Response;
use chrono::{DateTime, Utc};
use dioxus::prelude::Result;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, TextMergeStream};
use regex::Regex;
use rss::{ChannelBuilder, Item};
use surrealdb::engine::remote::http::Client;
use surrealdb::Surreal;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

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

pub struct RenderedMarkdown {
    pub html: String,
    pub sections: Vec<Section>,
}

pub async fn process_markdown(markdown: String) -> Result<String> {
    Ok(render_markdown(markdown).await?.html)
}

pub async fn render_markdown(markdown: String) -> Result<RenderedMarkdown> {
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

    let re_img = Regex::new(r"!\[(.*?)\]\((.*?\.(svg|png|jpe?g|gif|bmp|webp))\)")?;

    let mut processed_markdown = String::new();
    let mut last_img_end = 0;
    for img_cap in re_img.captures_iter(&markdown) {
        processed_markdown.push_str(&markdown[last_img_end..img_cap.get(0).unwrap().start()]);
        processed_markdown.push_str(&image_html(&img_cap[2], &img_cap[1]));
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
    // Source URL and alt text of the image whose inner events are being collected.
    let mut pending_image: Option<(String, String)> = None;

    for event in iterator {
        if pending_image.is_some() {
            match event {
                Event::Text(text) | Event::Code(text) => {
                    if let Some((_, alt)) = pending_image.as_mut() {
                        alt.push_str(&text);
                    }
                }
                Event::End(TagEnd::Image) => {
                    if let Some((src, alt)) = pending_image.take() {
                        events.push(Event::Html(CowStr::from(image_html(&src, &alt))));
                    }
                }
                _ => {}
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
                events.push(Event::Html(CowStr::from(highlight_code(&code_block_content, language)?)));
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
                pending_image = Some((dest_url.into_string(), String::new()));
            }
            other if !in_code_block => events.push(other),
            _ => {}
        }
    }

    // The page renders the post title as its only h1, so posts written with `#` sections move down a level.
    let has_h1 = events.iter().any(|event| {
        matches!(
            event,
            Event::Start(Tag::Heading {
                level: HeadingLevel::H1,
                ..
            })
        )
    });
    if has_h1 {
        for event in events.iter_mut() {
            if let Event::Start(Tag::Heading { level, .. }) | Event::End(TagEnd::Heading(level)) = event {
                *level = HeadingLevel::try_from(*level as usize + 1).unwrap_or(HeadingLevel::H6);
            }
        }
    }

    let sections = assign_heading_ids(&mut events);

    use pulldown_cmark::html::push_html;
    let mut html_output = String::new();
    push_html(&mut html_output, events.into_iter());

    Ok(RenderedMarkdown {
        html: html_output,
        sections,
    })
}

fn syntax_set() -> &'static SyntaxSet {
    static SYNTAXES: OnceLock<SyntaxSet> = OnceLock::new();
    SYNTAXES.get_or_init(SyntaxSet::load_defaults_newlines)
}

pub fn highlight_code(code: &str, language: &str) -> Result<String> {
    let syntaxes = syntax_set();
    let syntax = syntaxes
        .find_syntax_by_token(language)
        .unwrap_or_else(|| syntaxes.find_syntax_plain_text());
    let mut generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, syntaxes, ClassStyle::SpacedPrefixed { prefix: "sx-" });
    for line in LinesWithEndings::from(code) {
        generator.parse_html_for_line_which_includes_newline(line)?;
    }
    Ok(format!(r#"<pre class="code-block"><code>{}</code></pre>"#, generator.finalize()))
}

/// The first Rust block of a post (else its first block), capped at `max_lines` lines.
pub fn first_code_block(markdown: &str, max_lines: usize) -> Option<(String, String)> {
    let mut blocks: Vec<(String, String)> = Vec::new();
    let mut current: Option<(String, String)> = None;
    for event in Parser::new_ext(markdown, Options::empty()) {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                let language = match kind {
                    CodeBlockKind::Fenced(info) => info.split_whitespace().next().unwrap_or("").to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                current = Some((language, String::new()));
            }
            Event::Text(text) => {
                if let Some((_, code)) = current.as_mut() {
                    code.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => blocks.extend(current.take()),
            _ => {}
        }
    }
    let pick = blocks
        .iter()
        .position(|(language, _)| matches!(language.as_str(), "rust" | "rs"))
        .unwrap_or(0);
    let (language, code) = blocks.into_iter().nth(pick)?;
    let code = code.lines().take(max_lines).collect::<Vec<_>>().join("\n");
    let language = if language.is_empty() {
        "plaintext".to_string()
    } else {
        language
    };
    Some((language, code))
}

fn assign_heading_ids(events: &mut [Event]) -> Vec<Section> {
    let mut used = HashSet::new();
    let mut sections = Vec::new();
    let mut index = 0;
    while index < events.len() {
        let Event::Start(Tag::Heading { level, .. }) = &events[index] else {
            index += 1;
            continue;
        };
        let level = *level;
        let mut title = String::new();
        let mut end = index + 1;
        while end < events.len() && !matches!(events[end], Event::End(TagEnd::Heading(_))) {
            if let Event::Text(text) | Event::Code(text) = &events[end] {
                title.push_str(text);
            }
            end += 1;
        }
        let id = unique_id(slugify(&title), &mut used);
        if let Event::Start(Tag::Heading { id: heading_id, .. }) = &mut events[index] {
            *heading_id = Some(CowStr::from(id.clone()));
        }
        if level == HeadingLevel::H2 {
            sections.push(Section {
                id,
                title: title.trim().to_string(),
            });
        }
        index = end + 1;
    }
    sections
}

fn slugify(text: &str) -> String {
    let mut slug = String::new();
    for ch in text.chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() {
            slug.push(ch);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_end_matches('-');
    if slug.is_empty() {
        "section".to_string()
    } else {
        slug.to_string()
    }
}

fn unique_id(base: String, used: &mut HashSet<String>) -> String {
    let mut id = base.clone();
    let mut suffix = 2;
    while !used.insert(id.clone()) {
        id = format!("{base}-{suffix}");
        suffix += 1;
    }
    id
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
    for (path, lastmod) in [("/", latest_change), ("/projects", None), ("/opensource", None)] {
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
    let robots = format!("User-agent: *\nAllow: /\n\nSitemap: {}\n", absolute_url("/sitemap.xml"));
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

fn image_html(src: &str, alt: &str) -> String {
    // SVG diagrams are drawn black on transparent, so the dark page inverts them through this class.
    let class = if src.to_lowercase().ends_with(".svg") {
        r#" class="invert-on-dark""#
    } else {
        ""
    };
    format!(
        r#"<div style="display: flex; justify-content: center;"><img src="{}" alt="{}" loading="lazy" decoding="async"{class} style="width: 100%;"></div>"#,
        escape_attribute(src),
        escape_attribute(alt),
    )
}

fn escape_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn images_keep_their_alt_text_and_load_lazily() {
        let html = process_markdown(
            "![ESP32 \"pinout\"](https://cdn.example/pinout.png)\n\n![Flow](https://cdn.example/flow)".into(),
        )
        .await
        .unwrap();

        assert!(html.contains(r#"alt="ESP32 &quot;pinout&quot;" loading="lazy""#));
        assert!(html.contains(r#"<img src="https://cdn.example/flow" alt="Flow" loading="lazy""#));
    }

    #[tokio::test]
    async fn posts_with_h1_sections_are_shifted_below_the_page_title() {
        let html = process_markdown("# Intro\n\n## Details\n\ntext".into()).await.unwrap();

        assert!(html.contains(r#"<h2 id="intro">Intro</h2>"#));
        assert!(html.contains(r#"<h3 id="details">Details</h3>"#));
        assert!(!html.contains("<h1>"));
    }

    #[tokio::test]
    async fn svg_images_are_inverted_only_through_the_dark_theme_class() {
        let html = process_markdown("![Equation](https://cdn.example/eq.svg)".into())
            .await
            .unwrap();

        assert!(html.contains(r#"class="invert-on-dark""#));
        assert!(!html.contains("invert(100%)"));
    }

    #[tokio::test]
    async fn posts_without_h1_keep_their_heading_levels() {
        let html = process_markdown("## Details\n\ntext".into()).await.unwrap();

        assert!(html.contains(r#"<h2 id="details">Details</h2>"#));
    }

    #[tokio::test]
    async fn headings_get_unique_ids_and_h2s_become_sections() {
        let rendered = render_markdown("## Setup\n\n### Details\n\n## Setup\n\ntext".into()).await.unwrap();

        assert!(rendered.html.contains(r#"<h2 id="setup">Setup</h2>"#));
        assert!(rendered.html.contains(r#"<h3 id="details">Details</h3>"#));
        assert!(rendered.html.contains(r#"<h2 id="setup-2">Setup</h2>"#));
        let ids: Vec<&str> = rendered.sections.iter().map(|section| section.id.as_str()).collect();
        assert_eq!(ids, ["setup", "setup-2"]);
    }

    #[tokio::test]
    async fn shifted_h1_sections_are_listed() {
        let rendered = render_markdown("# Intro\n\n## Details".into()).await.unwrap();

        assert!(rendered.html.contains(r#"<h2 id="intro">Intro</h2>"#));
        assert_eq!(rendered.sections.len(), 1);
        assert_eq!(rendered.sections[0].title, "Intro");
    }

    #[tokio::test]
    async fn heading_ids_keep_letters_and_drop_symbols() {
        let rendered = render_markdown("## Part 1 — `Fgn::new` in Eötvös & co.\n\n## ***".into())
            .await
            .unwrap();

        assert_eq!(rendered.sections[0].id, "part-1-fgn-new-in-eötvös-co");
        assert_eq!(rendered.sections[0].title, "Part 1 — Fgn::new in Eötvös & co.");
        assert_eq!(rendered.sections[1].id, "section");
    }

    #[tokio::test]
    async fn code_blocks_are_highlighted_with_classes() {
        let html = process_markdown("```rust\nfn main() {}\n```".into()).await.unwrap();

        assert!(html.contains(r#"<pre class="code-block"><code>"#));
        assert!(html.contains("sx-"));
        assert!(!html.contains("style="));
    }

    #[test]
    fn unknown_languages_are_escaped_as_plain_text() {
        let html = highlight_code("<b>x</b>\n", "nonsense").unwrap();

        assert!(html.contains("&lt;b&gt;x&lt;/b&gt;"));
    }

    #[test]
    fn first_code_block_prefers_rust_and_caps_lines() {
        let markdown = "```toml\n[dependencies]\n```\n\n```rust\nfn a() {}\nfn b() {}\nfn c() {}\n```";

        assert_eq!(
            first_code_block(markdown, 2),
            Some(("rust".to_string(), "fn a() {}\nfn b() {}".to_string()))
        );
    }

    #[test]
    fn first_code_block_falls_back_to_the_first_block_or_none() {
        assert_eq!(
            first_code_block("```toml\na = 1\n```", 20),
            Some(("toml".to_string(), "a = 1".to_string()))
        );
        assert_eq!(
            first_code_block("    indented\n", 20),
            Some(("plaintext".to_string(), "indented".to_string()))
        );
        assert_eq!(first_code_block("no code here", 20), None);
    }
}
