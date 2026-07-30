pub mod api;
#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod redirect;
#[cfg(feature = "server")]
pub mod server_utils;
pub mod types;

#[cfg(feature = "server")]
pub mod app_state {
    use tokio::sync::OnceCell;

    use crate::ssr::db::{connect, Db};

    // Caches the self-healing handle, never a connection: `Db` swaps its own
    // inner connection out when the session or token dies, so this cell can
    // stay set for the process lifetime without going stale.
    static DB_CELL: OnceCell<Db> = OnceCell::const_new();

    pub async fn init_db() {
        let _ = db().await;
    }

    pub async fn db() -> Db {
        DB_CELL
            .get_or_init(|| async { connect().await.expect("failed to connect to surrealdb") })
            .await
            .clone()
    }
}
