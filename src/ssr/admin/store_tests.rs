use super::*;
use std::process::{Child, Command, Stdio};
use surrealdb::{
    engine::remote::http::Http,
    opt::auth::{Database, Root},
};

struct TestDb {
    server: Child,
    db: Surreal<Client>,
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let _ = self.server.kill();
        let _ = self.server.wait();
    }
}

async fn setup() -> TestDb {
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let server = Command::new("surreal")
        .args([
            "start",
            "--bind",
            &format!("127.0.0.1:{port}"),
            "--user",
            "root",
            "--pass",
            "root",
            "memory",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut test = TestDb {
        server,
        db: Surreal::init(),
    };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
    while std::net::TcpStream::connect(("127.0.0.1", port)).is_err() {
        assert!(
            std::time::Instant::now() < deadline,
            "Local test database did not start"
        );
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let root = Surreal::new::<Http>(format!("127.0.0.1:{port}")).await.unwrap();
    root.signin(Root {
        username: "root".into(),
        password: "root".into(),
    })
    .await
    .unwrap();
    root.use_ns("admin_test").use_db("admin_test").await.unwrap();
    root.query(include_str!("../../../database/schema/author.surql"))
        .await
        .unwrap()
        .check()
        .unwrap();
    root.query(include_str!("../../../database/schema/post.surql"))
        .await
        .unwrap()
        .check()
        .unwrap();
    root.query("CREATE author:daniel SET name = 'Daniel Boros', email = 'info@rust-dd.com'; DEFINE USER blog_svc ON DATABASE PASSWORD 'test' ROLES EDITOR;").await.unwrap().check().unwrap();
    test.db = Surreal::new::<Http>(format!("127.0.0.1:{port}")).await.unwrap();
    test.db.use_ns("admin_test").use_db("admin_test").await.unwrap();
    test.db
        .signin(Database {
            namespace: "admin_test".into(),
            database: "admin_test".into(),
            username: "blog_svc".into(),
            password: "test".into(),
        })
        .await
        .unwrap();
    test
}

fn input() -> PostInput {
    PostInput {
        title: "A Rust post".into(),
        summary: "Summary".into(),
        body: "## Introduction\n\n```rust\nfn main() {}\n```".into(),
        author_id: "daniel".into(),
        topic: Some("web".into()),
        tags: vec!["Rust".into()],
        ..PostInput::default()
    }
}

#[tokio::test]
async fn public_read_hides_drafts_and_unpublished_posts() {
    let test = setup().await;
    let post = save(&test.db, None, input()).await.unwrap();
    let slug = post.slug.clone().unwrap();
    assert!(crate::ssr::api::load_published_post(&test.db, slug.clone())
        .await
        .unwrap()
        .is_none());
    publish(&test.db, post.id.clone(), true).await.unwrap();
    assert!(crate::ssr::api::load_published_post(&test.db, slug.clone())
        .await
        .unwrap()
        .is_some());
    publish(&test.db, post.id, false).await.unwrap();
    assert!(crate::ssr::api::load_published_post(&test.db, slug)
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn draft_save_publication_and_deletion_use_the_service_account() {
    let test = setup().await;
    let draft = save(&test.db, None, input()).await.unwrap();
    assert!(!draft.is_published);
    assert_eq!(draft.slug.as_deref(), Some("a-rust-post"));
    assert_eq!(authors(&test.db).await.unwrap().len(), 1);
    assert_eq!(list(&test.db).await.unwrap().len(), 1);
    let mut changed = draft.input.clone();
    changed.body.push_str("\n\nA saved edit.");
    let saved = save(&test.db, Some(draft.id.clone()), changed.clone()).await.unwrap();
    assert_eq!(saved.input, changed);
    assert!(!saved.is_published);
    let published = publish(&test.db, draft.id.clone(), true).await.unwrap();
    assert!(published.is_published);
    let saved = save(&test.db, Some(draft.id.clone()), changed).await.unwrap();
    assert!(saved.is_published);
    assert_eq!(saved.created_at, published.created_at);
    let unpublished = publish(&test.db, draft.id.clone(), false).await.unwrap();
    assert!(!unpublished.is_published);
    let republished = publish(&test.db, draft.id.clone(), true).await.unwrap();
    assert_eq!(republished.created_at, published.created_at);
    delete(&test.db, draft.id.clone()).await.unwrap();
    assert!(load(&test.db, draft.id).await.is_err());
    assert!(list(&test.db).await.unwrap().is_empty());
}

#[tokio::test]
async fn missing_posts_and_authors_are_rejected_without_creating_records() {
    let test = setup().await;
    assert!(publish(&test.db, "missing".into(), true).await.is_err());
    assert!(delete(&test.db, "missing".into()).await.is_err());
    assert!(save(&test.db, Some("missing".into()), input()).await.is_err());
    let mut invalid = input();
    invalid.author_id = "missing".into();
    assert!(save(&test.db, None, invalid).await.is_err());
    assert!(list(&test.db).await.unwrap().is_empty());
}
