use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PostInput {
    pub title: String,
    pub summary: String,
    pub body: String,
    pub author_id: String,
    pub topic: Option<String>,
    pub tags: Vec<String>,
    pub header_image: Option<String>,
    pub show_cta: bool,
}

impl PostInput {
    pub fn validate(&self) -> std::result::Result<(), String> {
        for (name, value, max) in [
            ("Title", self.title.as_str(), 300),
            ("Summary", self.summary.as_str(), 2_000),
            ("Markdown", self.body.as_str(), 1_000_000),
            ("Author", self.author_id.as_str(), 256),
        ] {
            if value.trim().is_empty() || value.len() > max {
                return Err(format!("{name} is required and must be at most {max} bytes."));
            }
        }
        if self
            .topic
            .as_ref()
            .is_some_and(|topic| !crate::topics::TOPICS.iter().any(|item| item.name == topic))
        {
            return Err("Choose one of the available topics.".into());
        }
        if self.tags.len() > 30 || self.tags.iter().any(|tag| tag.trim().is_empty() || tag.len() > 100) {
            return Err("Use at most 30 tags, each between 1 and 100 bytes.".into());
        }
        if let Some(image) = &self.header_image {
            let valid = image
                .parse::<http::Uri>()
                .ok()
                .is_some_and(|uri| uri.scheme_str() == Some("https") && uri.host().is_some() && !image.contains('@'));
            if !valid || image.len() > 2_000 {
                return Err("The header image must be a valid HTTPS URL.".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AdminPost {
    pub id: String,
    pub input: PostInput,
    pub is_published: bool,
    pub slug: Option<String>,
    pub created_at: String,
    pub revision: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminAuthor {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminSession {
    pub configured: bool,
    pub authenticated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_post() -> PostInput {
        PostInput {
            title: "Writing Rust".into(),
            summary: "A short introduction".into(),
            body: "## Hello\n\nContent".into(),
            author_id: "daniel".into(),
            topic: Some("web".into()),
            ..PostInput::default()
        }
    }

    #[test]
    fn empty_or_oversized_content_is_rejected() {
        let mut input = valid_post();
        assert!(input.validate().is_ok());
        input.title = "  ".into();
        assert!(input.validate().is_err());
        input = valid_post();
        input.body = "x".repeat(1_000_001);
        assert!(input.validate().is_err());
    }

    #[test]
    fn invalid_topics_authors_and_image_urls_are_rejected() {
        let mut input = valid_post();
        input.topic = Some("unknown".into());
        assert!(input.validate().is_err());
        input = valid_post();
        input.author_id.clear();
        assert!(input.validate().is_err());
        input = valid_post();
        input.header_image = Some("javascript:alert(1)".into());
        assert!(input.validate().is_err());
        input.header_image = Some("https://cdn.example/image.png".into());
        assert!(input.validate().is_ok());
    }
}
