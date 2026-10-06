#[cfg(feature = "server")]
pub mod auth;
#[cfg(feature = "server")]
pub mod middleware;
#[cfg(feature = "server")]
pub(crate) mod store;
pub mod types;

#[cfg(feature = "server")]
use dioxus::fullstack::ServerFnError;
use dioxus::fullstack::ServerFnResult;
use dioxus::prelude::*;
use types::{AdminAuthor, AdminPost, AdminSession, PostInput};

#[cfg(feature = "server")]
async fn connection() -> tokio::sync::OwnedRwLockReadGuard<surrealdb::Surreal<surrealdb::engine::remote::http::Client>>
{
    crate::ssr::app_state::db().await.get().await
}

#[get("/api/admin/session")]
pub async fn admin_session() -> ServerFnResult<AdminSession> {
    #[cfg(feature = "server")]
    {
        Ok(AdminSession {
            configured: auth::Config::from_env().is_some(),
            authenticated: auth::require_admin().is_ok(),
        })
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[post("/api/admin/login")]
pub async fn admin_login(password: String) -> ServerFnResult<()> {
    #[cfg(feature = "server")]
    {
        auth::login(password).await.map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[post("/api/admin/logout")]
pub async fn admin_logout() -> ServerFnResult<()> {
    #[cfg(feature = "server")]
    {
        auth::logout().map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[get("/api/admin/posts")]
pub async fn admin_posts() -> ServerFnResult<Vec<AdminPost>> {
    #[cfg(feature = "server")]
    {
        auth::require_admin().map_err(server_error)?;
        store::list(&*connection().await).await.map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[get("/api/admin/authors")]
pub async fn admin_authors() -> ServerFnResult<Vec<AdminAuthor>> {
    #[cfg(feature = "server")]
    {
        auth::require_admin().map_err(server_error)?;
        store::authors(&*connection().await).await.map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[get("/api/admin/posts/{id}")]
pub async fn admin_post(id: String) -> ServerFnResult<AdminPost> {
    #[cfg(feature = "server")]
    {
        auth::require_admin().map_err(server_error)?;
        store::load(&*connection().await, id).await.map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[post("/api/admin/posts/save")]
pub async fn admin_save(id: Option<String>, input: PostInput) -> ServerFnResult<AdminPost> {
    #[cfg(feature = "server")]
    {
        let config = auth::require_admin().map_err(server_error)?;
        auth::require_origin(&config).map_err(server_error)?;
        store::save(&*connection().await, id, input).await.map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[post("/api/admin/posts/{id}/publication")]
pub async fn admin_publish(id: String, published: bool) -> ServerFnResult<AdminPost> {
    #[cfg(feature = "server")]
    {
        let config = auth::require_admin().map_err(server_error)?;
        auth::require_origin(&config).map_err(server_error)?;
        store::publish(&*connection().await, id, published)
            .await
            .map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[post("/api/admin/posts/{id}/delete")]
pub async fn admin_delete(id: String) -> ServerFnResult<()> {
    #[cfg(feature = "server")]
    {
        let config = auth::require_admin().map_err(server_error)?;
        auth::require_origin(&config).map_err(server_error)?;
        store::delete(&*connection().await, id).await.map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[post("/api/admin/preview")]
pub async fn admin_preview(body: String) -> ServerFnResult<String> {
    #[cfg(feature = "server")]
    {
        let config = auth::require_admin().map_err(server_error)?;
        auth::require_origin(&config).map_err(server_error)?;
        if body.len() > 1_000_000 {
            return Err(server_error(auth::error(400, "The Markdown is too long to preview.")));
        }
        crate::ssr::markdown::process_markdown(body).await.map_err(server_error)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[cfg(feature = "server")]
fn server_error(error: dioxus::CapturedError) -> ServerFnError {
    if let Some(error) = error.downcast_ref::<ServerFnError>() {
        return error.clone();
    }
    tracing::error!(%error, "Admin request failed");
    ServerFnError::ServerError {
        code: 500,
        message: "The request failed. Try again shortly.".into(),
        details: None,
    }
}
