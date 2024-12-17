pub mod api;
#[cfg(feature = "ssr")]
pub mod redirect;
#[cfg(feature = "ssr")]
pub mod server_utils;

#[cfg(feature = "ssr")]
pub mod app_state {
    use axum::extract::FromRef;
    use leptos::prelude::*;
    use surrealdb::{engine::remote::http::Client, Surreal};

    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub db: Surreal<Client>,
        pub leptos_options: LeptosOptions,
    }
}
