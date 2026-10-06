use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
};
use http::{header, HeaderValue};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct ClientIp(pub String);

pub async fn protect(mut request: Request, next: Next) -> Response {
    let path = request.uri().path();
    let admin = path == "/admin" || path.starts_with("/admin/") || path.starts_with("/api/admin/");
    if !admin {
        return next.run(request).await;
    }
    let mut ip = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(address)| address.ip().to_string())
        .unwrap_or_else(|| "unknown".into());
    if std::env::var("ADMIN_TRUST_PROXY").as_deref() == Ok("true") {
        if let Some(forwarded) = request
            .headers()
            .get("x-forwarded-for")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.rsplit(',').next())
            .and_then(|value| value.trim().parse::<std::net::IpAddr>().ok())
        {
            ip = forwarded.to_string();
        }
    }
    request.extensions_mut().insert(ClientIp(ip));
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store, private"));
    headers.insert("x-robots-tag", HeaderValue::from_static("noindex, nofollow"));
    headers.insert(header::REFERRER_POLICY, HeaderValue::from_static("same-origin"));
    headers.insert("x-frame-options", HeaderValue::from_static("DENY"));
    response
}
