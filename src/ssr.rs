pub mod api;
#[cfg(feature = "server")]
pub mod redirect;
#[cfg(feature = "server")]
pub mod server_utils;
pub mod types;

#[cfg(feature = "server")]
pub mod app_state {
    use surrealdb::{engine::remote::http::Client, Surreal};
    use tokio::sync::OnceCell;

    use crate::ssr::server_utils::connect;

    pub static DB_CELL: OnceCell<Surreal<Client>> = OnceCell::const_new();

    pub async fn init_db() {
        let _ = DB_CELL.get_or_init(connect).await;
    }

    pub async fn db() -> Surreal<Client> {
        init_db().await;
        DB_CELL.get().expect("db not initialized").clone()
    }
}
