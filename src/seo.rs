use dioxus::prelude::*;
use serde_json::{json, Value};

use crate::ssr::types::Post;

pub const SITE_NAME: &str = "Rust-DD";
pub const HOME_TITLE: &str = "Rust-DD: Practical Rust Engineering Blog & Consulting";
pub const SITE_DESCRIPTION: &str = "Practical Rust engineering notes on async, SIMD, CUDA, embedded and quant finance, backed by benchmarks and code. Plus open-source crates and Rust consulting.";
pub const SITE_URL: &str = "https://rust-dd.com";
pub const DEFAULT_OG_IMAGE: &str = "https://rust-dd.com/og-image.png";
const DEFAULT_OG_IMAGE_WIDTH: &str = "1200";
const DEFAULT_OG_IMAGE_HEIGHT: &str = "637";
const DEFAULT_OG_IMAGE_ALT: &str = "rust-dd wordmark above Ferris the Rust crab";
const LOGO_URL: &str = "https://rust-dd.com/logo.png";
pub const X_HANDLE: &str = "@rust_dd";
const SOCIAL_PROFILES: &[&str] = &[
    "https://github.com/rust-dd",
    "https://x.com/rust_dd",
    "https://www.linkedin.com/company/rust-dd",
];

const MAX_TITLE_CHARS: usize = 60;
const MAX_DESCRIPTION_CHARS: usize = 160;

pub fn absolute_url(path: &str) -> String {
    let normalized = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };

    format!("{SITE_URL}{normalized}")
}

/// Appends the site name only while the result fits the ~60 characters a search result shows.
pub fn page_title(title: &str) -> String {
    let branded = format!("{title} | {SITE_NAME}");
    if branded.chars().count() <= MAX_TITLE_CHARS {
        branded
    } else {
        title.to_string()
    }
}

/// Collapses whitespace and cuts at a word boundary, since search snippets stop around 160 characters.
pub fn meta_description(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let full = words.join(" ");
    if full.chars().count() <= MAX_DESCRIPTION_CHARS {
        return full;
    }

    let mut cut = String::new();
    for word in words {
        // Leaves room for the separating space and the trailing ellipsis.
        if cut.chars().count() + word.chars().count() + 2 > MAX_DESCRIPTION_CHARS {
            break;
        }
        if !cut.is_empty() {
            cut.push(' ');
        }
        cut.push_str(word);
    }

    format!("{}…", cut.trim_end_matches(|c: char| c.is_ascii_punctuation()))
}

#[component]
pub fn PageMeta(
    #[props(into)] title: String,
    #[props(into)] description: String,
    /// Site-relative path of the page, e.g. `/projects`.
    #[props(into)]
    path: String,
    #[props(into, default = "website".to_string())] og_type: String,
    /// Falls back to the site-wide share image.
    image: Option<String>,
    #[props(default)] noindex: bool,
) -> Element {
    let url = absolute_url(&path);
    let custom_image = image.is_some();
    let image = image.unwrap_or_else(|| DEFAULT_OG_IMAGE.to_string());

    rsx! {
        document::Title { "{title}" }
        document::Meta { name: "description", content: "{description}" }
        if noindex {
            document::Meta { name: "robots", content: "noindex" }
        } else {
            document::Link { rel: "canonical", href: "{url}" }
        }
        document::Meta { property: "og:type", content: "{og_type}" }
        document::Meta { property: "og:title", content: "{title}" }
        document::Meta { property: "og:description", content: "{description}" }
        document::Meta { property: "og:url", content: "{url}" }
        document::Meta { property: "og:image", content: "{image}" }
        if custom_image {
            document::Meta { property: "og:image:alt", content: "{title}" }
        } else {
            document::Meta { property: "og:image:width", content: DEFAULT_OG_IMAGE_WIDTH }
            document::Meta { property: "og:image:height", content: DEFAULT_OG_IMAGE_HEIGHT }
            document::Meta { property: "og:image:alt", content: DEFAULT_OG_IMAGE_ALT }
        }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
    }
}

#[component]
pub fn JsonLd(value: Value) -> Element {
    // SSR writes script text unescaped, so a `</script>` inside a post title would end the tag early.
    let json = value.to_string().replace('<', "\\u003c");

    rsx! {
        document::Script { r#type: "application/ld+json", "{json}" }
    }
}

fn organization() -> Value {
    json!({
        "@type": "Organization",
        "@id": format!("{SITE_URL}/#organization"),
        "name": SITE_NAME,
        "url": absolute_url("/"),
        "logo": LOGO_URL,
        "sameAs": SOCIAL_PROFILES,
    })
}

pub fn website_graph() -> Value {
    json!({
        "@context": "https://schema.org",
        "@graph": [
            organization(),
            {
                "@type": "WebSite",
                "@id": format!("{SITE_URL}/#website"),
                "url": absolute_url("/"),
                "name": SITE_NAME,
                "alternateName": "rust-dd",
                "description": SITE_DESCRIPTION,
                "inLanguage": "en",
                "publisher": { "@id": format!("{SITE_URL}/#organization") },
            },
        ],
    })
}

pub fn blog_posting(post: &Post, url: &str, image: &str) -> Value {
    let author = &post.author;
    let profiles: Vec<&str> = [&author.linkedin, &author.github, &author.twitter]
        .into_iter()
        .flatten()
        .map(String::as_str)
        .collect();
    let mut person = json!({ "@type": "Person", "name": author.name });
    if let Some(profile) = profiles.first() {
        person["url"] = json!(profile);
        person["sameAs"] = json!(profiles);
    }
    let keywords: Vec<&str> = post
        .tags
        .iter()
        .map(|tag| tag.trim())
        .filter(|tag| !tag.is_empty())
        .collect();

    json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "@id": format!("{url}#article"),
        "mainEntityOfPage": url,
        "headline": post.title,
        "description": post.summary,
        "image": image,
        "datePublished": post.created_at,
        "dateModified": post.content_updated_at.as_deref().unwrap_or(&post.created_at),
        "author": person,
        "publisher": organization(),
        "keywords": keywords,
        "inLanguage": "en",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_description_only_collapses_whitespace() {
        assert_eq!(meta_description("  Fast\nasync   Rust. "), "Fast async Rust.");
    }

    #[test]
    fn long_description_is_cut_at_a_word_boundary() {
        let summary = "word, ".repeat(60);
        let description = meta_description(&summary);

        assert!(description.chars().count() <= MAX_DESCRIPTION_CHARS);
        assert!(description.ends_with("word…"));
    }

    #[test]
    fn page_title_brands_only_titles_that_still_fit() {
        assert_eq!(
            page_title("Inline Assembly in Rust"),
            "Inline Assembly in Rust | Rust-DD"
        );

        let long = "Building a Rust library for DHT11 sensor: A Step-by-Step Guide";
        assert_eq!(page_title(long), long);
    }
}
