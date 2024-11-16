#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::extract::State;
    use axum::response::Response;
    use axum::{routing::get, Router};
    use blog::api::{process_markdown, Post};
    use blog::app::{shell, App};
    use blog::redirect::redirect_www;
    use blog::ssr::AppState;
    use chrono::{DateTime, Utc};
    use dotenvy::dotenv;
    use leptos::logging;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::env;
    use surrealdb::engine::remote::http::Client;
    use surrealdb::{
        engine::remote::http::{Http, Https},
        opt::auth::Root,
        Surreal,
    };
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

    async fn generate_rss(db: Surreal<Client>) -> Result<String, ServerFnError> {
        use rss::{ChannelBuilder, Item};
        use std::sync::Arc;
        use tokio::sync::Mutex;

        let query = format!(
            "SELECT *, author.* from post WHERE is_published = true ORDER BY created_at DESC;"
        );
        let query = db.query(&query).await;
        let mut posts = query?.take::<Vec<Post>>(0)?;
        posts.iter_mut().for_each(|post| {
            let date_time = DateTime::parse_from_rfc3339(&post.created_at)
                .unwrap()
                .with_timezone(&Utc);
            let naive_date = date_time.date_naive();
            let formatted_date = naive_date.format("%b %-d").to_string();
            post.created_at = formatted_date.into();
        });
        let posts = Arc::new(Mutex::new(posts));
        let mut handles = vec![];

        for _ in 0..posts.lock().await.len() {
            let posts_clone = Arc::clone(&posts);
            let handle = tokio::spawn(async move {
                let mut posts = posts_clone.lock().await;
                if let Some(post) = posts.iter_mut().next() {
                    post.body = process_markdown(post.body.to_string())
                        .await
                        .unwrap()
                        .into();
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let channel = ChannelBuilder::default()
            .title("Rust-DD")
            .link("https://rust-dd.com")
            .description("Tech Diaries - The Official Rust-DD Developer Blog")
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

    async fn rss_handler(State(state): State<AppState>) -> Response<String> {
        let AppState { db, .. } = state;
        let rss = generate_rss(db).await.unwrap();
        Response::builder()
            .header("Content-Type", "application/xml")
            .body(rss)
            .unwrap()
    }

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
