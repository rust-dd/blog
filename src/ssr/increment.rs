use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use axum::extract::State;
use regex::Regex;
use crate::ssr::app_state::AppState;
use crate::ssr::types::Post;

pub async fn increment(State(app_state): State<AppState>, req: Request<Body>, next: Next) -> Result<Response<Body>, StatusCode> {
    let uri = req.uri();
    let uri = uri.to_string();
    let re = Regex::new(r"^post/.*").unwrap();
    tracing::info!("Incrementing views for: {}", uri);

    if !re.is_match(&uri) {
        tracing::info!("Not a post, skipping incrementing views");
        return Ok(next.run(req).await);
    }

    let slug = uri.split("/").last().unwrap().to_string();
    tracing::info!("Incrementing views for post: {}", slug);
    let AppState { db, .. } = app_state;
    let query = format!(r#"
        BEGIN TRANSACTION;
        LET $post = array::first(SELECT id from post WHERE slug = "{slug}");
        UPDATE $post.id SET total_views += 1;
        COMMIT TRANSACTION;
    "#);
    tracing::info!("Query: {}", query);
    let query = db.query(&query).await;
    tracing::info!("Query result: {:?}", query);
    if let Err(e) = query {
        tracing::error!("Error incrementing views: {:?}", e);
        return Ok(next.run(req).await);
    }

    tracing::info!("Incremented views for post: {}", slug);
    Ok(next.run(req).await)
}
