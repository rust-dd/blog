use super::types::Post;
use crate::ssr::app_state::AppState;
use axum::extract::State;
use axum::response::Response;
use chrono::{DateTime, Utc};
use leptos::prelude::ServerFnError;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, TextMergeStream};
use regex::Regex;
use rss::{ChannelBuilder, Item};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use surrealdb::engine::remote::http::{Client, Http, Https};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
use tokio::sync::Mutex;

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

pub async fn rss_handler(State(state): State<AppState>) -> Response<String> {
    let AppState { db, .. } = state;
    let rss = generate_rss(db).await.unwrap();
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(rss)
        .unwrap()
}

pub async fn generate_rss(db: Surreal<Client>) -> Result<String, ServerFnError> {
    let query = db
        .query("SELECT *, author.* from post WHERE is_published = true ORDER BY created_at DESC;")
        .await;
    let mut posts = query?.take::<Vec<Post>>(0)?;
    posts.iter_mut().for_each(|post| {
        let date_time = DateTime::parse_from_rfc3339(&post.created_at)
            .unwrap()
            .with_timezone(&Utc);
        let naive_date = date_time.date_naive();
        let formatted_date = naive_date.format("%b %-d").to_string();
        post.created_at = formatted_date.into();
    });
    let posts = Arc::new(Mutex::new(posts));
    let mut handles = vec![];

    for _ in 0..posts.lock().await.len() {
        let posts_clone = Arc::clone(&posts);
        let handle = tokio::spawn(async move {
            let mut posts = posts_clone.lock().await;
            if let Some(post) = posts.iter_mut().next() {
                post.body = process_markdown(post.body.to_string()).await.unwrap().into();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    let channel = ChannelBuilder::default()
        .title("Rust-DD")
        .link("https://rust-dd.com")
        .description("Rust-DD Blog – Tech Insights & Consulting")
        .items(
            posts
                .lock()
                .await
                .clone()
                .into_iter()
                .map(|post| {
                    let mut item = Item::default();
                    item.set_author(post.author.name.to_string());
                    item.set_title(post.title.to_string());
                    item.set_description(post.body.to_string());
                    item.set_link(format!("https://rust-dd.com/post/{}", post.slug.unwrap()));
                    item.set_pub_date(post.created_at.to_string());
                    item
                })
                .collect::<Vec<_>>(),
        )
        .build();

    Ok(channel.to_string())
}

pub async fn process_markdown(markdown: String) -> Result<String, ServerFnError> {
    pub struct MathEventProcessor {
        display_style_opts: katex::Opts,
    }

    impl MathEventProcessor {
        pub fn new() -> MathEventProcessor {
            let opts = katex::Opts::builder().display_mode(true).build().unwrap();
            MathEventProcessor {
                display_style_opts: opts,
            }
        }

        pub fn process_math_event<'a>(&'a self, event: Event<'a>) -> Event<'a> {
            match event {
                Event::InlineMath(math_exp) => Event::InlineHtml(CowStr::from(katex::render(&math_exp).unwrap())),
                Event::DisplayMath(math_exp) => Event::Html(CowStr::from(
                    katex::render_with_opts(&math_exp, &self.display_style_opts).unwrap(),
                )),
                _ => event,
            }
        }
    }

    // Initialize syntax highlighting
    let ps = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-eighties.dark"];

    // Regex for images
    let re_img = Regex::new(r"!\[.*?\]\((.*?\.(svg|png|jpe?g|gif|bmp|webp))\)")?;

    // Preprocess the markdown to handle images
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

    // Now parse the markdown
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_MATH);

    let parser = Parser::new_ext(&processed_markdown, options);

    // Initialize the MathEventProcessor
    let mep = MathEventProcessor::new();

    // Prepare to collect the events
    let mut events = Vec::new();

    let mut code_block_language: Option<String> = None;
    let mut code_block_content = String::new();
    let mut in_code_block = false;
    let mut skip_image = false;

    // Use TextMergeStream to merge adjacent text events
    let iterator = TextMergeStream::new(parser).map(|event| mep.process_math_event(event));

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

                // Extract language from CodeBlockKind
                code_block_language = match kind {
                    CodeBlockKind::Fenced(info) => {
                        // Get the first word as the language identifier
                        let lang = info.split_whitespace().next().unwrap_or("").to_string();
                        Some(lang)
                    }
                    CodeBlockKind::Indented => None,
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;

                // Perform syntax highlighting on the code block content
                let language = code_block_language.as_deref().unwrap_or("plaintext");
                let syntax = ps
                    .find_syntax_by_token(language)
                    .unwrap_or_else(|| ps.find_syntax_plain_text());
                let mut h = HighlightLines::new(syntax, theme);
                let mut highlighted_html = String::with_capacity(code_block_content.len() * 2);
                highlighted_html
                    .push_str(r#"<pre style="background-color: #2b303b; padding: 8px; border-radius: 8px"><code>"#);

                for line in code_block_content.lines() {
                    let ranges = h.highlight_line(line, &ps)?;
                    let escaped = styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No)?;
                    highlighted_html.push_str(&escaped);
                    highlighted_html.push('\n');
                }
                highlighted_html.push_str("</code></pre>");

                events.push(Event::Html(CowStr::from(highlighted_html)));

                code_block_language = None;
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else {
                    events.push(Event::Text(text));
                }
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                // Handle the image
                let img_path = dest_url.into_string();
                let img_format = img_path.split('.').last().unwrap_or("").to_lowercase();

                let img_html = if img_format == "svg" {
                    format!(
                        r#"<div style="display: flex; justify-content: center;"><img alt="iamge" src="{}" style="filter: invert(100%); width: 100%;"></div>"#,
                        img_path
                    )
                } else {
                    format!(
                        r#"<div style="display: flex; justify-content: center;"><img alt="iamge" src="{}" style="width: 100%;"></div>"#,
                        img_path
                    )
                };

                events.push(Event::Html(CowStr::from(img_html)));

                // Set skip_image flag to true to skip alt text and End(TagEnd::Image)
                skip_image = true;
            }
            Event::End(TagEnd::Image) => {
                // This will be skipped when skip_image is true
                if !skip_image {
                    events.push(Event::End(TagEnd::Image));
                }
            }
            other => {
                events.push(other);
            }
        }
    }

    // Render the events back to HTML
    use pulldown_cmark::html::push_html;
    let mut html_output = String::new();
    push_html(&mut html_output, events.into_iter());

    Ok(html_output)
}

pub async fn sitemap_handler(State(state): State<AppState>) -> Response<String> {
    #[derive(Serialize, Deserialize)]
    struct Post {
        slug: Option<String>,
        created_at: String,
    }

    let AppState { db, .. } = state;
    let query = db
        .query("SELECT slug, created_at FROM post WHERE is_published = true ORDER BY created_at DESC;")
        .await;
    let posts = query.unwrap().take::<Vec<Post>>(0).unwrap();
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
        sitemap.push_str("<url>\n");
        sitemap.push_str(&format!("<loc>https://rust-dd.com/post/{}</loc>\n", post.slug.unwrap()));
        sitemap.push_str("<changefreq>monthly</changefreq>\n");
        sitemap.push_str("<priority>1.0</priority>\n");
        sitemap.push_str(&format!("<lastmod>{}</lastmod>\n", post.created_at));
        sitemap.push_str("</url>\n");
    }
    sitemap.push_str("</urlset>");
    Response::builder()
        .header("Content-Type", "application/xml")
        .body(sitemap)
        .unwrap()
}
