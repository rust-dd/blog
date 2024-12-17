#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router};
    use blog::app::{shell, App};
    use blog::ssr::app_state::AppState;
    use blog::ssr::redirect::redirect_www;
    use blog::ssr::server_utils::{connect, rss_handler, sitemap_handler};
    use dotenvy::dotenv;
    use leptos::logging;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::compression::predicate::{NotForContentType, SizeAbove};
    use tower_http::compression::{CompressionLayer, Predicate};
    use tower_http::trace::TraceLayer;
    use tower_http::CompressionLevel;

    let tracing_level = if cfg!(debug_assertions) {
        tracing::Level::INFO
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing_level)
        .init();

    let env_result = dotenv();
    if env_result.is_err() {
        logging::warn!("There is no corresponding .env file");
    }

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let db = connect().await;
    let app_state = AppState {
        db,
        leptos_options: leptos_options.clone(),
    };

    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .route("/rss.xml", get(rss_handler))
        .route("/sitemap.xml", get(sitemap_handler))
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
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
