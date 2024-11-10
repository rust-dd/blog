use std::{borrow::Cow, collections::BTreeMap};

use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub id: Thing,
    pub name: Cow<'static, str>,
    pub email: Cow<'static, str>,
    pub bio: Option<Cow<'static, str>>,
    pub linkedin: Option<Cow<'static, str>>,
    pub twitter: Option<Cow<'static, str>>,
    pub github: Option<Cow<'static, str>>,
}

impl Default for Author {
    fn default() -> Self {
        Self {
            id: Thing::from(("author", "0")),
            name: Cow::Borrowed(""),
            email: Cow::Borrowed(""),
            bio: None,
            linkedin: None,
            twitter: None,
            github: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub id: Thing,
    pub title: Cow<'static, str>,
    pub summary: Cow<'static, str>,
    pub body: Cow<'static, str>,
    pub tags: Vec<Cow<'static, str>>,
    pub author: Author,
    pub read_time: usize,
    pub total_views: usize,
    pub slug: Option<Cow<'static, str>>,
    pub created_at: Cow<'static, str>,
    pub updated_at: Cow<'static, str>,
    pub is_published: bool,
    pub header_image: Option<Cow<'static, str>>,
}

impl<'a> Default for Post {
    fn default() -> Self {
        Self {
            id: Thing::from(("post", "0")),
            title: Cow::Borrowed(""),
            summary: Cow::Borrowed(""),
            body: Cow::Borrowed(""),
            tags: vec![],
            author: Author::default(),
            read_time: 0,
            total_views: 0,
            slug: None,
            created_at: Cow::Borrowed(""),
            updated_at: Cow::Borrowed(""),
            is_published: true,
            header_image: None,
        }
    }
}

#[server(endpoint = "/posts")]
pub async fn select_posts(
    #[server(default)] tags: Vec<String>,
) -> Result<Vec<Post>, ServerFnError> {
    use crate::ssr::AppState;
    use chrono::{DateTime, Utc};
    use leptos::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();
    let mut query =
        format!("SELECT *, author.* from post WHERE is_published = true ORDER BY created_at DESC;");
    if !tags.is_empty() {
        let tags = tags
            .iter()
            .map(|tag| format!(r#""{}""#, tag))
            .collect::<Vec<_>>();
        query = format!(
            "SELECT *, author.* from post WHERE tags CONTAINSANY [{0}] ORDER BY created_at DESC;",
            tags.join(", ")
        );
    }

    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    let mut posts = query?.take::<Vec<Post>>(0)?;
    posts.iter_mut().for_each(|post| {
        let date_time = DateTime::parse_from_rfc3339(&post.created_at)
            .unwrap()
            .with_timezone(&Utc);
        let naive_date = date_time.date_naive();
        let formatted_date = naive_date.format("%b %-d, %Y").to_string();
        post.created_at = formatted_date.into();
    });

    Ok(posts)
}

#[server(endpoint = "/tags")]
pub async fn select_tags() -> Result<BTreeMap<String, usize>, ServerFnError> {
    use crate::ssr::AppState;
    use leptos::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();

    let query = format!(
        "
    LET $tags = SELECT tags FROM post;
    array::flatten($tags.map(|$t| $t.tags));
    "
    );
    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    let tags = query?.take::<Vec<String>>(1)?;
    let mut tag_map = BTreeMap::<String, usize>::new();
    for tag in tags {
        *tag_map.entry(tag).or_insert(0) += 1;
    }

    Ok(tag_map)
}

#[server(endpoint = "/post")]
pub async fn select_post(slug: String) -> Result<Post, ServerFnError> {
    use crate::ssr::AppState;
    use chrono::{DateTime, Utc};
    use leptos::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();

    let query = format!(r#"SELECT *, author.* from post WHERE slug = "{slug}""#);
    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    let post = query?.take::<Vec<Post>>(0)?;
    let mut post = post.first().unwrap().clone();

    let date_time = DateTime::parse_from_rfc3339(&post.created_at)?.with_timezone(&Utc);
    let naive_date = date_time.date_naive();
    let formatted_date = naive_date.format("%b %-d").to_string();
    post.created_at = formatted_date.into();
    post.body = process_markdown(post.body.to_string())
        .await
        .unwrap()
        .into();

    Ok(post)
}

#[server(endpoint = "/increment_views")]
pub async fn increment_views(id: String) -> Result<(), ServerFnError> {
    use crate::ssr::AppState;
    use leptos::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();

    let query = format!("UPDATE post:{0} SET total_views = total_views + 1;", id);
    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    Ok(())
}

#[server]
pub async fn process_markdown(markdown: String) -> Result<String, ServerFnError> {
    use pulldown_cmark::{
        CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, TextMergeStream,
    };
    use regex::Regex;
    use syntect::easy::HighlightLines;
    use syntect::highlighting::ThemeSet;
    use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
    use syntect::parsing::SyntaxSet;

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
                Event::InlineMath(math_exp) => {
                    Event::InlineHtml(CowStr::from(katex::render(&math_exp).unwrap()))
                }
                Event::DisplayMath(math_exp) => Event::Html(CowStr::from(
                    katex::render_with_opts(&math_exp, &self.display_style_opts).unwrap(),
                )),
                _ => event,
            }
        }
    }

    // Initialize syntax highlighting
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-eighties.dark"];

    // Regex for images
    let re_img = Regex::new(r"!\[.*?\]\((.*?\.(svg|png|jpe?g|gif|bmp|webp))\)").unwrap();

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
                highlighted_html.push_str(
                    r#"<pre style="background-color: #2b303b; padding: 8px; border-radius: 8px"><code>"#,
                );

                for line in code_block_content.lines() {
                    let ranges = h.highlight_line(line, &ps).unwrap();
                    let escaped =
                        styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No)
                            .unwrap();
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
