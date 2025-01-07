use app::types::{AppState, Post};
use axum::extract::State;
use axum::response::Response;
use chrono::{DateTime, Utc};
use leptos::prelude::ServerFnError;
use markdown::process_markdown;
use rss::{ChannelBuilder, Item};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use surrealdb::engine::remote::http::{Client, Http, Https};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
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
    tracing::info!("Connected to SurrealDB");
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

pub async fn generate_rss(db: Surreal<Client>) -> leptos::error::Result<String, ServerFnError> {
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
        post.created_at = formatted_date;
    });
    let posts = Arc::new(Mutex::new(posts));
    let mut handles = vec![];

    for _ in 0..posts.lock().await.len() {
        let posts_clone = Arc::clone(&posts);
        let handle = tokio::spawn(async move {
            let mut posts = posts_clone.lock().await;
            if let Some(post) = posts.iter_mut().next() {
                post.body = process_markdown(post.body.to_string()).await.unwrap();
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
