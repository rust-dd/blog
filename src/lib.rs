#![feature(async_closure)]

pub mod api;
pub mod app;
pub mod error_template;
pub mod home;
pub mod loader;
pub mod post;
#[cfg(feature = "ssr")]
pub mod redirect;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use axum::extract::FromRef;
    use leptos::prelude::*;
    use surrealdb::{engine::remote::http::Client, Surreal};

    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub db: Surreal<Client>,
        pub leptos_options: LeptosOptions,
    }
}
