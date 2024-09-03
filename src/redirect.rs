use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next
};

pub async fn redirect_www(req: Request<Body>, next: Next) -> Result<Response<Body>, StatusCode> {
    if let Some(host) = req.headers().get("host") {
        if let Ok(host) = host.to_str() {
            if host.starts_with("www.") {
                let new_host = host.trim_start_matches("www.");
                let new_uri = format!("https://{}{}", new_host, req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));
                let response = Response::builder()
                    .status(StatusCode::MOVED_PERMANENTLY)
                    .header("location", new_uri)
                    .body(Body::empty())
                    .unwrap();
                return Ok(response);
            }
        }
    }
    Ok(next.run(req).await)
}