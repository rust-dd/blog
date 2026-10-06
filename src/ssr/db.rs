use std::env;
use std::sync::Arc;
use std::time::Duration;

use surrealdb::engine::remote::http::{Client, Http, Https};
use surrealdb::opt::auth::{Database, Root};
use surrealdb::Surreal;
use tokio::sync::{OwnedRwLockReadGuard, RwLock};

#[derive(Clone)]
pub struct DbConfig {
    pub protocol: String,
    pub host: String,
    pub ns: String,
    pub db: String,
    pub auth: DbAuth,
}

#[derive(Clone)]
pub enum DbAuth {
    Service { user: String, pass: String },
    Root { user: String, pass: String },
}

impl DbConfig {
    pub fn from_env() -> Self {
        // The database-level service user decouples the app from cloud-managed
        // instance credentials, which the provider may rotate; root signin
        // stays as the local-dev fallback.
        let auth = match (env::var("SURREAL_USER"), env::var("SURREAL_PASS")) {
            (Ok(user), Ok(pass)) => DbAuth::Service { user, pass },
            _ => DbAuth::Root {
                user: env::var("SURREAL_ROOT_USER").unwrap_or_else(|_| "root".into()),
                pass: env::var("SURREAL_ROOT_PASS").unwrap_or_else(|_| "root".into()),
            },
        };
        Self {
            protocol: env::var("SURREAL_PROTOCOL").unwrap_or_else(|_| "http".into()),
            host: env::var("SURREAL_HOST").unwrap_or_else(|_| "127.0.0.1:8000".into()),
            ns: env::var("SURREAL_NS").unwrap_or_else(|_| "rustblog".into()),
            db: env::var("SURREAL_DB").unwrap_or_else(|_| "rustblog".into()),
            auth,
        }
    }
}

/// Self-healing handle around the HTTP connection. The server-side session
/// dies for good on a Surreal Cloud restart or a `DEFINE USER OVERWRITE`
/// (the SDK cannot recover the handle, not even with a fresh signin), so
/// callers must fetch the current connection per use instead of caching it.
#[derive(Clone)]
pub struct Db {
    current: Arc<RwLock<Surreal<Client>>>,
}

impl Db {
    /// Borrows the current connection. A guard (not a clone) on purpose:
    /// cloning a `Surreal` opens a new server-side session and replays
    /// signin, which would add several round trips per use.
    pub async fn get(&self) -> OwnedRwLockReadGuard<Surreal<Client>> {
        Arc::clone(&self.current).read_owned().await
    }
}

pub async fn connect() -> surrealdb::Result<Db> {
    connect_with(DbConfig::from_env()).await
}

pub async fn connect_with(config: DbConfig) -> surrealdb::Result<Db> {
    let first = open(&config).await?;
    let db = Db {
        current: Arc::new(RwLock::new(first)),
    };
    tokio::spawn(watchdog(db.clone(), config));
    Ok(db)
}

const PROBE_INTERVAL: Duration = Duration::from_secs(5);

// A Surreal Cloud restart drops the server-side session ("Session not found")
// and a DEFINE USER OVERWRITE rotates the token signing key (401); neither is
// recoverable on the existing handle, so a failed probe swaps in a freshly
// opened connection. When the server itself is down, open() fails too and the
// old handle is kept for the next tick.
async fn watchdog(db: Db, config: DbConfig) {
    loop {
        tokio::time::sleep(PROBE_INTERVAL).await;
        let probe = db.get().await.query("RETURN 1").await;
        let Err(err) = probe else { continue };
        tracing::warn!("surrealdb probe failed, reconnecting: {err}");
        match open(&config).await {
            Ok(fresh) => {
                *db.current.write().await = fresh;
                tracing::info!("surrealdb connection replaced");
            }
            Err(err) => tracing::error!("surrealdb reconnect failed: {err}"),
        }
    }
}

async fn open(config: &DbConfig) -> surrealdb::Result<Surreal<Client>> {
    let db = if config.protocol == "https" {
        Surreal::new::<Https>(config.host.clone()).await?
    } else {
        Surreal::new::<Http>(config.host.clone()).await?
    };
    match &config.auth {
        DbAuth::Service { user, pass } => {
            db.signin(Database {
                namespace: config.ns.clone(),
                database: config.db.clone(),
                username: user.clone(),
                password: pass.clone(),
            })
            .await?;
        }
        DbAuth::Root { user, pass } => {
            db.signin(Root {
                username: user.clone(),
                password: pass.clone(),
            })
            .await?;
        }
    }
    db.use_ns(config.ns.clone()).use_db(config.db.clone()).await?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use std::process::{Child, Command, Stdio};
    use std::time::{Duration, Instant};

    use super::*;

    struct ServerGuard(Child);

    impl ServerGuard {
        // SIGTERM, not SIGKILL: a graceful shutdown removes the node's
        // server-side sessions (matching a Surreal Cloud restart), which is
        // the exact failure being reproduced.
        fn stop_gracefully(mut self) {
            let _ = Command::new("kill").arg(self.0.id().to_string()).status();
            let _ = self.0.wait();
        }
    }

    impl Drop for ServerGuard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    fn spawn_server(port: u16, dir: &std::path::Path) -> ServerGuard {
        let child = Command::new("surreal")
            .args(["start", "--bind"])
            .arg(format!("127.0.0.1:{port}"))
            .args(["--user", "root", "--pass", "root"])
            .arg(format!("rocksdb:{}", dir.display()))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to spawn surreal");
        ServerGuard(child)
    }

    async fn wait_ready(port: u16) {
        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            if std::net::TcpStream::connect(("127.0.0.1", port)).is_ok() {
                return;
            }
            assert!(Instant::now() < deadline, "surreal did not become ready");
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }

    fn free_port() -> u16 {
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    fn surreal_binary_missing() -> bool {
        Command::new("surreal").arg("version").output().is_err()
    }

    async fn bootstrap(port: u16, dir: &std::path::Path) -> (ServerGuard, Surreal<Client>, Db) {
        let addr = format!("127.0.0.1:{port}");
        let server = spawn_server(port, dir);
        wait_ready(port).await;

        let root = Surreal::new::<Http>(addr.clone()).await.unwrap();
        root.signin(Root {
            username: "root".into(),
            password: "root".into(),
        })
        .await
        .unwrap();
        root.query("DEFINE NAMESPACE IF NOT EXISTS test")
            .await
            .unwrap()
            .check()
            .unwrap();
        root.use_ns("test").use_db("test").await.unwrap();
        root.query("DEFINE DATABASE IF NOT EXISTS test")
            .await
            .unwrap()
            .check()
            .unwrap();
        root.query(
            "DEFINE USER IF NOT EXISTS blog_svc ON DATABASE PASSWORD 'pw' \
             ROLES EDITOR DURATION FOR TOKEN 1y, FOR SESSION NONE",
        )
        .await
        .unwrap()
        .check()
        .unwrap();

        let db = connect_with(DbConfig {
            protocol: "http".into(),
            host: addr,
            ns: "test".into(),
            db: "test".into(),
            auth: DbAuth::Service {
                user: "blog_svc".into(),
                pass: "pw".into(),
            },
        })
        .await
        .unwrap();
        db.get().await.query("RETURN 1").await.unwrap();

        (server, root, db)
    }

    async fn assert_recovers(db: &Db, scenario: &str) {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            match db.get().await.query("RETURN 1").await {
                Ok(_) => break,
                Err(err) => eprintln!("probe failed: {err}"),
            }
            assert!(
                Instant::now() < deadline,
                "connection did not recover within 30s after {scenario}"
            );
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    #[tokio::test]
    async fn connection_recovers_after_server_restart() {
        if surreal_binary_missing() {
            eprintln!("surreal binary not found; skipping");
            return;
        }
        let port = free_port();
        let dir = std::env::temp_dir().join(format!("blog-reconnect-test-{port}"));

        let (server, _root, db) = bootstrap(port, &dir).await;

        server.stop_gracefully();
        let _server = spawn_server(port, &dir);
        wait_ready(port).await;
        // Real outages leave the stale session idle for minutes before the
        // next request; probing instantly can still hit it pre-cleanup.
        tokio::time::sleep(Duration::from_secs(10)).await;

        assert_recovers(&db, "server restart").await;
        let _ = std::fs::remove_dir_all(&dir);
    }

    // Every surrealkit schema sync re-runs DEFINE USER OVERWRITE, which
    // rotates the user's token signing key and 401s the live connection.
    #[tokio::test]
    async fn connection_recovers_after_user_overwrite() {
        if surreal_binary_missing() {
            eprintln!("surreal binary not found; skipping");
            return;
        }
        let port = free_port();
        let dir = std::env::temp_dir().join(format!("blog-overwrite-test-{port}"));

        let (_server, root, db) = bootstrap(port, &dir).await;

        root.query(
            "DEFINE USER OVERWRITE blog_svc ON DATABASE PASSWORD 'pw' \
             ROLES EDITOR DURATION FOR TOKEN 1y, FOR SESSION NONE",
        )
        .await
        .unwrap()
        .check()
        .unwrap();

        assert_recovers(&db, "user overwrite").await;
        let _ = std::fs::remove_dir_all(&dir);
    }
}
