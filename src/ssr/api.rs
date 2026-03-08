use std::collections::BTreeMap;

use crate::ssr::types::Post;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use serde::Deserialize;

#[cfg(feature = "server")]
use std::time::{Duration, Instant};

#[cfg(feature = "server")]
use tokio::sync::RwLock;

#[cfg(feature = "server")]
const REPO_STARS_CACHE_TTL: Duration = Duration::from_secs(60 * 60);

#[cfg(feature = "server")]
static REPO_STARS_CACHE: RwLock<Option<RepoStarsCache>> = RwLock::const_new(None);

#[cfg(feature = "server")]
#[derive(Clone)]
struct RepoStarsCache {
    fetched_at: Instant,
    stars: BTreeMap<String, u32>,
}

#[cfg(feature = "server")]
#[derive(Deserialize)]
struct GithubRepo {
    stargazers_count: u32,
}

#[cfg(feature = "server")]
async fn fetch_repo_stars_from_github() -> BTreeMap<String, u32> {
    use std::env;

    use crate::pages::opensource::PROJECTS;
    use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github+json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-dd-blog"));
    headers.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));

    if let Ok(token) = env::var("GITHUB_TOKEN") {
        if let Ok(value) = HeaderValue::from_str(&format!("Bearer {token}")) {
            headers.insert(AUTHORIZATION, value);
        }
    }

    let client = match reqwest::Client::builder().default_headers(headers).build() {
        Ok(client) => client,
        Err(_) => return BTreeMap::new(),
    };

    let mut stars = BTreeMap::new();

    for project in PROJECTS.iter() {
        let Some((owner, repo)) = project.github_repo.split_once('/') else {
            continue;
        };

        let response = match client
            .get(format!("https://api.github.com/repos/{owner}/{repo}"))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => response,
            _ => continue,
        };

        let payload = match response.json::<GithubRepo>().await {
            Ok(payload) => payload,
            Err(_) => continue,
        };

        stars.insert(project.github_repo.to_string(), payload.stargazers_count);
    }

    stars
}

#[get("/api/github/stars")]
pub async fn select_repo_stars() -> Result<BTreeMap<String, u32>> {
    #[cfg(feature = "server")]
    {
        let now = Instant::now();

        {
            let cache = REPO_STARS_CACHE.read().await;
            if let Some(cache) = cache.as_ref() {
                if now.duration_since(cache.fetched_at) < REPO_STARS_CACHE_TTL {
                    return Ok(cache.stars.clone());
                }
            }
        }

        let stars = fetch_repo_stars_from_github().await;

        let mut cache = REPO_STARS_CACHE.write().await;
        *cache = Some(RepoStarsCache {
            fetched_at: now,
            stars: stars.clone(),
        });

        Ok(stars)
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[get("/api/posts")]
pub async fn select_posts() -> Result<Vec<Post>> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;
        use chrono::{DateTime, Utc};

        let db = db().await;
        let mut query = db
            .query("SELECT *, author.* from post WHERE is_published = true ORDER BY created_at DESC;")
            .await?;

        let mut posts = query.take::<Vec<Post>>(0)?;
        posts.iter_mut().for_each(|post| {
            let date_time = DateTime::parse_from_rfc3339(&post.created_at)
                .unwrap()
                .with_timezone(&Utc);
            let naive_date = date_time.date_naive();
            let formatted_date = naive_date.format("%b %-d, %Y").to_string();
            post.created_at = formatted_date;
        });

        Ok(posts)
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[get("/api/tags")]
pub async fn select_tags() -> Result<BTreeMap<String, usize>> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;

        let db = db().await;
        let mut query = db
            .query(
                "
        LET $tags = SELECT tags FROM post;
        array::flatten($tags.map(|$t| $t.tags));
        ",
            )
            .await?;

        let tags = query.take::<Vec<String>>(1)?;
        let mut tag_map = BTreeMap::<String, usize>::new();
        for tag in tags {
            *tag_map.entry(tag).or_insert(0) += 1;
        }

        Ok(tag_map)
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[get("/api/post/{slug}")]
pub async fn select_post(slug: String) -> Result<Post> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;
        use crate::ssr::server_utils::process_markdown;
        use chrono::{DateTime, Utc};

        let db = db().await;
        let mut query = db
            .query(format!(r#"SELECT *, author.* from post WHERE slug = "{slug}""#))
            .await?;
        let post = query.take::<Vec<Post>>(0)?;
        let mut post = match post.first().cloned() {
            Some(post) => post,
            None => return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "post not found").into()),
        };

        let date_time = DateTime::parse_from_rfc3339(&post.created_at)?.with_timezone(&Utc);
        let naive_date = date_time.date_naive();
        let formatted_date = naive_date.format("%b %-d").to_string();
        post.created_at = formatted_date;
        post.body = process_markdown(post.body.clone()).await?;

        Ok(post)
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[post("/api/posts/{id}/increment_views")]
pub async fn increment_views(id: String) -> Result<()> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;

        let db = db().await;
        db.query(format!("UPDATE post:{0} SET total_views = total_views + 1;", id))
            .await?;

        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}
