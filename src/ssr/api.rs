use std::collections::BTreeMap;

use crate::ssr::types::Post;
use dioxus::prelude::*;

#[get("/api/posts")]
pub async fn select_posts() -> Result<Vec<Post>> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;
        use chrono::{DateTime, Utc};

        let db = db().await;
        let mut query = db
            .query("SELECT *, author.* from post WHERE is_published = true ORDER BY created_at DESC;")
            .await?;

        let mut posts = query.take::<Vec<Post>>(0)?;
        posts.iter_mut().for_each(|post| {
            let date_time = DateTime::parse_from_rfc3339(&post.created_at)
                .unwrap()
                .with_timezone(&Utc);
            let naive_date = date_time.date_naive();
            let formatted_date = naive_date.format("%b %-d, %Y").to_string();
            post.created_at = formatted_date;
        });

        Ok(posts)
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[get("/api/tags")]
pub async fn select_tags() -> Result<BTreeMap<String, usize>> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;

        let db = db().await;
        let mut query = db
            .query(
                "
        LET $tags = SELECT tags FROM post;
        array::flatten($tags.map(|$t| $t.tags));
        ",
            )
            .await?;

        let tags = query.take::<Vec<String>>(1)?;
        let mut tag_map = BTreeMap::<String, usize>::new();
        for tag in tags {
            *tag_map.entry(tag).or_insert(0) += 1;
        }

        Ok(tag_map)
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[get("/api/post/{slug}")]
pub async fn select_post(slug: String) -> Result<Post> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;
        use crate::ssr::server_utils::process_markdown;
        use chrono::{DateTime, Utc};

        let db = db().await;
        let mut query = db
            .query(format!(r#"SELECT *, author.* from post WHERE slug = "{slug}""#))
            .await?;
        let post = query.take::<Vec<Post>>(0)?;
        let mut post = match post.first().cloned() {
            Some(post) => post,
            None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "post not found").into()),
        };

        let date_time = DateTime::parse_from_rfc3339(&post.created_at)?.with_timezone(&Utc);
        let naive_date = date_time.date_naive();
        let formatted_date = naive_date.format("%b %-d").to_string();
        post.created_at = formatted_date;
        post.body = process_markdown(post.body.clone()).await?;

        Ok(post)
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[post("/api/posts/{id}/increment_views")]
pub async fn increment_views(id: String) -> Result<()> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;

        let db = db().await;
        db.query(format!("UPDATE post:{0} SET total_views = total_views + 1;", id))
            .await?;

        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}
