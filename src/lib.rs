pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod home;
pub mod post;
pub mod posts;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use axum::extract::FromRef;
    use leptos::LeptosOptions;
    use surrealdb::{engine::remote::http::Client, Surreal};

    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub db: Surreal<Client>,
        pub leptos_options: LeptosOptions,
    }
}
