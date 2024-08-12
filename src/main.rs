#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::env;

    use axum::Router;
    use blog::app::App;
    use blog::fileserv::file_and_error_handler;
    use blog::ssr::AppState;
    use dotenvy::dotenv;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use surrealdb::{engine::remote::http::Http, opt::auth::Root, Surreal};

    let env_result = dotenv();
    if env_result.is_err() {
        logging::warn!("There is no corresponding .env file");
    }

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let host = env::var("SURREAL_HOST").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    let username = env::var("SURREAL_ROOT_USER").unwrap_or_else(|_| "root".to_string());
    let password = env::var("SURREAL_ROOT_PASS").unwrap_or_else(|_| "root".to_string());
    let ns = env::var("SURREAL_NS").unwrap_or_else(|_| "rustblog".to_string());
    let db_name = env::var("SURREAL_DB").unwrap_or_else(|_| "root".to_string());

    let db = Surreal::new::<Http>(host).await.unwrap();
    db.signin(Root {
        username: username.as_str(),
        password: password.as_str(),
    })
    .await
    .unwrap();
    db.use_ns(ns).use_db(db_name).await.unwrap();
    let app_state = AppState { db, leptos_options };

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            App,
        )
        .fallback(file_and_error_handler)
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
