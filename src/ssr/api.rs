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

    // Concurrent, so a cold cache costs the /opensource render one GitHub round trip, not one per repo.
    let mut requests = tokio::task::JoinSet::new();
    for project in PROJECTS.iter() {
        let Some((owner, repo)) = project.github_repo.split_once('/') else {
            continue;
        };
        let github_repo = project.github_repo;
        let request = client.get(format!("https://api.github.com/repos/{owner}/{repo}"));

        requests.spawn(async move {
            let response = request
                .send()
                .await
                .ok()
                .filter(|response| response.status().is_success())?;
            let payload = response.json::<GithubRepo>().await.ok()?;
            Some((github_repo, payload.stargazers_count))
        });
    }

    let mut stars = BTreeMap::new();
    while let Some(result) = requests.join_next().await {
        if let Ok(Some((github_repo, count))) = result {
            stars.insert(github_repo.to_string(), count);
        }
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

#[cfg(feature = "server")]
pub(crate) async fn published_posts() -> Result<Vec<Post>> {
    use crate::ssr::app_state::db;

    let db = db().await;
    let db = db.get().await;
    // Listings never render bodies, and every field returned here is embedded in the page's hydration data.
    let mut query = db
        .query("SELECT *, author.*, '' AS body, <string>created_at AS created_at, <string>updated_at AS updated_at FROM post WHERE is_published = true ORDER BY created_at DESC;")
        .await?;
    let mut posts = query.take::<Vec<Post>>(0)?;
    posts.iter_mut().for_each(hide_author_email);

    Ok(posts)
}

/// Author emails are personal inboxes that no page shows, so they never leave the server.
#[cfg(feature = "server")]
fn hide_author_email(post: &mut Post) {
    post.author.email.clear();
}

#[get("/api/posts")]
pub async fn select_posts() -> Result<Vec<Post>> {
    #[cfg(feature = "server")]
    {
        published_posts().await
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[get("/api/post/{slug}/related")]
pub async fn select_related_posts(slug: String) -> Result<Vec<Post>> {
    #[cfg(feature = "server")]
    {
        use std::collections::BTreeSet;

        const RELATED_POSTS: usize = 3;

        fn tags_of(post: &Post) -> BTreeSet<String> {
            post.tags
                .iter()
                .map(|tag| tag.trim().to_lowercase())
                .filter(|tag| !tag.is_empty())
                .collect()
        }

        let posts = published_posts().await?;
        let is_current = |post: &Post| post.slug.as_deref() == Some(slug.as_str());
        let Some(current_tags) = posts.iter().find(|post| is_current(post)).map(tags_of) else {
            return Ok(Vec::new());
        };

        let mut related: Vec<(usize, Post)> = posts
            .into_iter()
            .filter(|post| !is_current(post))
            .map(|post| (tags_of(&post).intersection(&current_tags).count(), post))
            .collect();
        // Stable sort, so posts sharing as many tags keep their newest-first order.
        related.sort_by_key(|(shared_tags, _)| std::cmp::Reverse(*shared_tags));

        Ok(related.into_iter().take(RELATED_POSTS).map(|(_, post)| post).collect())
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
        let db = db.get().await;
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

/// `Ok(None)` when no post has this slug, so the page can answer with a real 404.
#[get("/api/post/{slug}")]
pub async fn select_post(slug: String) -> Result<Option<Post>> {
    #[cfg(feature = "server")]
    {
        use crate::ssr::app_state::db;
        use crate::ssr::server_utils::process_markdown;

        let db = db().await;
        let db = db.get().await;
        let mut query = db
            .query("SELECT *, author.*, <string>created_at AS created_at, <string>updated_at AS updated_at FROM post WHERE slug = $slug")
            .bind(("slug", slug))
            .await?;
        let Some(mut post) = query.take::<Vec<Post>>(0)?.into_iter().next() else {
            return Ok(None);
        };
        hide_author_email(&mut post);
        post.body = process_markdown(post.body.clone()).await?;

        Ok(Some(post))
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
        use surrealdb_types::RecordId;

        let db = db().await;
        let db = db.get().await;
        db.query("UPDATE $post SET total_views = total_views + 1;")
            .bind(("post", RecordId::new("post", id)))
            .await?;

        Ok(())
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}
