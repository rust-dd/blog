#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use blog::app::{shell, App};
    use blog::ssr::AppState;
    use dotenvy::dotenv;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::env;
    use surrealdb::{
        engine::remote::http::{Http, Https},
        opt::auth::Root,
        Surreal,
    };
    use tower_http::compression::CompressionLayer;

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let env_result = dotenv();
    if env_result.is_err() {
        tracing::info!("There is no corresponding .env file");
    }

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

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
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state)
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
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
