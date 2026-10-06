use dioxus::prelude::Result;
use surrealdb::{engine::remote::http::Client, Surreal};
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue, Value};

use super::{
    auth::error,
    types::{AdminAuthor, AdminPost, PostInput},
};
use crate::ssr::{api::POST_FIELDS, types::Post};

fn key(record: RecordId) -> Result<String> {
    match record.key {
        RecordIdKey::String(key) => Ok(key),
        _ => Err(error(400, "This record has an unsupported identifier.")),
    }
}

fn admin_post(post: Post) -> Result<AdminPost> {
    Ok(AdminPost {
        id: key(post.id)?,
        input: PostInput {
            title: post.title,
            summary: post.summary,
            body: post.body,
            author_id: key(post.author.id)?,
            topic: post.topic,
            tags: post.tags,
            header_image: post.header_image,
            show_cta: post.show_cta,
        },
        is_published: post.is_published,
        slug: post.slug,
        created_at: post.created_at,
        revision: post.updated_at,
    })
}

pub async fn list(db: &Surreal<Client>) -> Result<Vec<AdminPost>> {
    let mut query = db
        .query(format!(
            "SELECT {POST_FIELDS}, '' AS body FROM post ORDER BY created_at DESC;"
        ))
        .await?;
    query.take::<Vec<Post>>(0)?.into_iter().map(admin_post).collect()
}

pub async fn authors(db: &Surreal<Client>) -> Result<Vec<AdminAuthor>> {
    #[derive(SurrealValue)]
    struct AuthorRow {
        id: RecordId,
        name: String,
    }
    let mut query = db.query("SELECT id, name FROM author ORDER BY name;").await?;
    query
        .take::<Vec<AuthorRow>>(0)?
        .into_iter()
        .map(|author| {
            Ok(AdminAuthor {
                id: key(author.id)?,
                name: author.name,
            })
        })
        .collect()
}

pub async fn load(db: &Surreal<Client>, id: String) -> Result<AdminPost> {
    let mut query = db
        .query(format!("SELECT {POST_FIELDS} FROM $post;"))
        .bind(("post", RecordId::new("post", id)))
        .await?;
    query
        .take::<Vec<Post>>(0)?
        .into_iter()
        .next()
        .ok_or_else(|| error(404, "Post not found."))
        .and_then(admin_post)
}

pub async fn save(db: &Surreal<Client>, id: Option<String>, input: PostInput) -> Result<AdminPost> {
    input.validate().map_err(|message| error(400, message))?;
    let author = RecordId::new("author", input.author_id.clone());
    let mut query = db
        .query("SELECT VALUE id FROM $author;")
        .bind(("author", author.clone()))
        .await?;
    if query.take::<Vec<RecordId>>(0)?.is_empty() {
        return Err(error(400, "Choose an existing author."));
    }
    let (id, operation) = match id {
        Some(id) => (id, "UPDATE"),
        None => {
            let mut bytes = [0u8; 16];
            getrandom::fill(&mut bytes).map_err(|_| error(500, "Could not create a post identifier."))?;
            (
                bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(),
                "CREATE",
            )
        }
    };
    db.query(format!("{operation} $post SET title = $title, summary = $summary, body = $body, author = $author, topic = $topic, tags = $tags, header_image = $header_image, show_cta = $show_cta;"))
        .bind(("post", RecordId::new("post", id.clone())))
        .bind(("title", input.title.trim().to_string())).bind(("summary", input.summary.trim().to_string()))
        .bind(("body", input.body)).bind(("author", author)).bind(("topic", input.topic))
        .bind(("tags", input.tags.into_iter().map(|tag| tag.trim().to_string()).collect::<Vec<_>>()))
        .bind(("header_image", input.header_image)).bind(("show_cta", input.show_cta))
        .await?.check()?;
    load(db, id).await
}

pub async fn publish(db: &Surreal<Client>, id: String, published: bool) -> Result<AdminPost> {
    db.query(
        "UPDATE $post SET \
        first_published_at = IF first_published_at = NONE AND is_published { created_at } ELSE { first_published_at }, \
        created_at = IF $published AND first_published_at = NONE { time::now() } ELSE { created_at }, \
        first_published_at = IF $published AND first_published_at = NONE { created_at } ELSE { first_published_at }, \
        is_published = $published;",
    )
    .bind(("post", RecordId::new("post", id.clone())))
    .bind(("published", published))
    .await?
    .check()?;
    load(db, id).await
}

pub async fn delete(db: &Surreal<Client>, id: String) -> Result<()> {
    let mut query = db
        .query("DELETE $post RETURN BEFORE;")
        .bind(("post", RecordId::new("post", id)))
        .await?;
    if query.take::<Vec<Value>>(0)?.is_empty() {
        return Err(error(404, "Post not found."));
    }
    Ok(())
}

#[cfg(test)]
#[path = "store_tests.rs"]
mod tests;
