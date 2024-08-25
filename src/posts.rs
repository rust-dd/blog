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
pub async fn select_posts() -> Result<Vec<Post>, ServerFnError> {
    use crate::ssr::AppState;
    use chrono::{DateTime, Utc};
    use leptos::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();

    let query = format!("SELECT *, author.* from post ORDER BY created_at DESC;");
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
