use std::collections::BTreeMap;

use leptos::prelude::{server, ServerFnError};
use serde::{Deserialize, Serialize};

use crate::ssr::types::{Post, Reference};

#[server(endpoint = "/posts")]
pub async fn select_posts(
    #[server(default)] tags: Vec<String>,
) -> Result<Vec<Post>, ServerFnError> {
    use crate::ssr::app_state::AppState;
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
    use crate::ssr::app_state::AppState;
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
    use super::server_utils::process_markdown;
    use crate::ssr::app_state::AppState;
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
    use crate::ssr::app_state::AppState;
    use leptos::prelude::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();

    let query = format!("UPDATE post:{0} SET total_views = total_views + 1;", id);
    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HireUsRequest {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

#[server(endpoint = "/hire_us")]
pub async fn hire_us(data: HireUsRequest) -> Result<(), ServerFnError> {
    use std::env;
    use lettre::{
        transport::smtp::authentication::Credentials,
        message::header::ContentType, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    };


    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&env::var("SMTP_HOST")?)
      ?
      .credentials(Credentials::new(
          env::var("SMTP_USER")?,
          env::var("SMTP_PASSWORD")?,
      ))
      .build::<Tokio1Executor>();

    let email = Message::builder()
        .from(data.email.parse()?)
        .to(env::var("SMTP_USER")?.parse()?)
        .subject(data.subject)
        .header(ContentType::TEXT_HTML)
        .body(data.message)
        .expect("failed to build email");

    match mailer.send(email).await {
        Ok(_) => {
            tracing::info!("Email sent successfully");
            return Ok(());
        }
        Err(e) => {
            tracing::error!("Failed to send email: {:?}", e);
            return Err(ServerFnError::from(e));
        }
    }
}

#[server(endpoint = "/references")]
pub async fn select_references() -> Result<Vec<Reference>, ServerFnError> {
    use crate::ssr::app_state::AppState;
    use leptos::prelude::expect_context;

    let AppState { db, .. } = expect_context::<AppState>();

    let query = "SELECT * from reference WHERE is_published = true ORDER BY created_at DESC;";
    let query = db.query(&query).await;

    if let Err(e) = query {
        return Err(ServerFnError::from(e));
    }

    let  references = query?.take::<Vec<Reference>>(0)?;
    Ok(references)
}