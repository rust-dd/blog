#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::routing::get;
    use blog::app::App;
    use blog::ssr::app_state::init_db;
    use blog::ssr::redirect::redirect_www;
    use blog::ssr::server_utils::{robots_handler, rss_handler, sitemap_handler};
    use dotenvy::dotenv;
    use tower_http::compression::predicate::{NotForContentType, SizeAbove};
    use tower_http::compression::{CompressionLayer, Predicate};
    use tower_http::trace::TraceLayer;
    use tower_http::CompressionLevel;

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    if dotenv().is_err() {
        tracing::warn!("There is no corresponding .env file");
    }

    init_db().await;

    let app = dioxus::server::router(App)
        .route("/rss.xml", get(rss_handler))
        .route("/sitemap.xml", get(sitemap_handler))
        .route("/robots.txt", get(robots_handler))
        .layer(
            tower::ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(axum::middleware::from_fn(redirect_www)),
        )
        .layer(
            CompressionLayer::new()
                .quality(CompressionLevel::Default)
                .compress_when(
                    SizeAbove::new(1500)
                        .and(NotForContentType::GRPC)
                        .and(NotForContentType::IMAGES)
                        .and(NotForContentType::const_new("application/xml"))
                        .and(NotForContentType::const_new("application/javascript"))
                        .and(NotForContentType::const_new("application/wasm"))
                        .and(NotForContentType::const_new("text/css")),
                ),
        );

    let addr = dioxus::cli_config::fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on http://{}", addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(blog::app::App);
}
