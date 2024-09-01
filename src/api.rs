use std::borrow::Cow;

use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

impl Default for Post {
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
    let mut query = format!("SELECT *, author.* from post ORDER BY created_at DESC;");
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
        let formatted_date = naive_date.format("%b %-d").to_string();
        post.created_at = formatted_date.into();
    });

    Ok(posts)
}

#[server(endpoint = "/tags")]
pub async fn select_tags() -> Result<Vec<String>, ServerFnError> {
    use crate::ssr::AppState;
    use leptos::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();

    let query = format!("SELECT tags from post;");
    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Tags {
        tags: Vec<String>,
    }
    let tags = query?.take::<Vec<Tags>>(0)?;
    let mut tags = tags.iter().flat_map(|t| t.tags.clone()).collect::<Vec<_>>();
    tags.sort();
    let tags = tags
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    Ok(tags)
}

#[server(endpoint = "/post")]
pub async fn select_post(slug: String) -> Result<Post, ServerFnError> {
    use crate::ssr::AppState;
    use cached::proc_macro::cached;
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

    #[cached]
    fn process_markdown(markdown: String) -> Result<String, ServerFnError> {
        use pulldown_cmark::{html::push_html, Options, Parser};
        use regex::Regex;
        use syntect::easy::HighlightLines;
        use syntect::highlighting::ThemeSet;
        use syntect::html::styled_line_to_highlighted_html;
        use syntect::html::IncludeBackground;
        use syntect::parsing::SyntaxSet;

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes["base16-eighties.dark"];

        let re = Regex::new(r"```(\w+)?\n([\s\S]*?)```").unwrap();

        let mut html_output = String::new();
        let mut last_end = 0;

        for cap in re.captures_iter(&markdown) {
            let markdown_before_code = &markdown[last_end..cap.get(0).unwrap().start()];
            let parser = Parser::new_ext(markdown_before_code, Options::all());
            push_html(&mut html_output, parser);

            let language = cap.get(1).map_or("plaintext", |m| m.as_str());
            let code_block = &cap[2];

            let syntax = ps
                .find_syntax_by_token(language)
                .unwrap_or_else(|| ps.find_syntax_plain_text());

            let mut h = HighlightLines::new(syntax, theme);
            html_output.push_str(r#"<pre style="background-color: #2b303b; padding: 8px; border-radius: 8px"><code>"#);

            for line in code_block.lines() {
                let ranges = h.highlight_line(line, &ps).unwrap();
                let escaped =
                    styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No).unwrap();
                html_output.push_str(&escaped);
                html_output.push_str("\n");
            }

            html_output.push_str("</code></pre>");

            last_end = cap.get(0).unwrap().end();
        }

        let markdown_after_last_code = &markdown[last_end..];
        let parser = Parser::new_ext(markdown_after_last_code, Options::all());
        push_html(&mut html_output, parser);

        Ok(html_output)
    }

    post.body = process_markdown(post.body.to_string()).unwrap().into();

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
