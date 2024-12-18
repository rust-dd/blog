use std::collections::BTreeMap;

use leptos::prelude::{server, ServerFnError};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub id: Thing,
    pub name: String,
    pub email: String,
    pub bio: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
}

impl Default for Author {
    fn default() -> Self {
        Self {
            id: Thing::from(("author", "0")),
            name: String::new(),
            email: String::new(),
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
    pub title: String,
    pub summary: String,
    pub body: String,
    pub tags: Vec<String>,
    pub author: Author,
    pub read_time: usize,
    pub total_views: usize,
    pub slug: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub is_published: bool,
    pub header_image: Option<String>,
}

impl<'a> Default for Post {
    fn default() -> Self {
        Self {
            id: Thing::from(("post", "0")),
            title: String::new(),
            summary: String::new(),
            body: String::new(),
            tags: vec![],
            author: Author::default(),
            read_time: 0,
            total_views: 0,
            slug: None,
            created_at: String::new(),
            updated_at: String::new(),
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
    use leptos::prelude::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();
    let mut query = String::from(
        "SELECT *, author.* from post WHERE is_published = true ORDER BY created_at DESC;",
    );
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
    use leptos::prelude::expect_context;

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
    use crate::server_utils::process_markdown;
    use crate::ssr::AppState;
    use chrono::{DateTime, Utc};
    use leptos::prelude::expect_context;

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
    post.body = process_markdown(post.body.to_string()).await?.into();

    Ok(post)
}

#[server(endpoint = "/increment_views")]
pub async fn increment_views(id: String) -> Result<(), ServerFnError> {
    use crate::ssr::AppState;
    use leptos::prelude::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();

    let query = format!("UPDATE post:{0} SET total_views = total_views + 1;", id);
    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    Ok(())
}
