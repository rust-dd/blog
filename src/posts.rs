use std::borrow::Cow;

use leptos::{logging::log, server, ServerFnError};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

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
            id: Thing {
                tb: "author".to_string(),
                id: "0".to_string().into(),
            },
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
    pub created_at: Cow<'static, str>,
    pub updated_at: Cow<'static, str>,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: Thing {
                tb: "post".to_string(),
                id: "0".to_string().into(),
            },
            title: Cow::Borrowed(""),
            summary: Cow::Borrowed(""),
            body: Cow::Borrowed(""),
            tags: vec![],
            author: Author::default(),
            read_time: 0,
            total_views: 0,
            created_at: Cow::Borrowed(""),
            updated_at: Cow::Borrowed(""),
        }
    }
}

#[server(endpoint = "/posts")]
pub async fn select_posts(offset: usize) -> Result<Vec<Post>, ServerFnError> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("rustblog").use_db("rustblog").await?;

    let query = format!("SELECT *, author.* from post LIMIT 20 START {0};", offset);

    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    let posts = query?.take::<Vec<Post>>(0)?;

    Ok(posts)
}

#[server(endpoint = "/post")]
pub async fn select_post(id: String) -> Result<Post, ServerFnError> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("rustblog").use_db("rustblog").await?;

    let query = format!("SELECT *, author.* from post:{0}", id);
    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    let post = query?.take::<Vec<Post>>(0)?;

    Ok(post.first().unwrap().clone())
}
