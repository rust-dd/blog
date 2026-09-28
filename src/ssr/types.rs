use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue, PartialEq)]
pub struct Author {
    pub id: RecordId,
    pub name: String,
    pub email: String,
    pub bio: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
}

impl Default for Author {
    fn default() -> Self {
        Self {
            id: RecordId::new("author", "0"),
            name: String::new(),
            email: String::new(),
            bio: None,
            linkedin: None,
            twitter: None,
            github: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue, PartialEq)]
pub struct Post {
    pub id: RecordId,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub tags: Vec<String>,
    pub author: Author,
    pub read_time: usize,
    pub total_views: usize,
    pub slug: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    /// Last change to the title, summary or body; `updated_at` also moves on view counts.
    pub content_updated_at: Option<String>,
    pub is_published: bool,
    pub header_image: Option<String>,
    pub show_cta: bool,
}

impl Post {
    /// `created_at` formatted for display, e.g. "Oct 1, 2024".
    pub fn published_on(&self) -> String {
        display_date(&self.created_at).unwrap_or_else(|| self.created_at.clone())
    }

    /// Display date of the last content edit, only when it fell on a later day than publishing.
    pub fn updated_on(&self) -> Option<String> {
        let updated = display_date(self.content_updated_at.as_deref()?)?;
        (updated != self.published_on()).then_some(updated)
    }
}

fn display_date(timestamp: &str) -> Option<String> {
    chrono::DateTime::parse_from_rfc3339(timestamp)
        .ok()
        .map(|date| date.with_timezone(&chrono::Utc).format("%b %-d, %Y").to_string())
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: RecordId::new("post", "0"),
            title: String::new(),
            summary: String::new(),
            body: String::new(),
            tags: vec![],
            author: Author::default(),
            read_time: 0,
            total_views: 0,
            slug: None,
            created_at: String::new(),
            updated_at: String::new(),
            content_updated_at: None,
            is_published: true,
            header_image: None,
            show_cta: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue, PartialEq)]
pub struct Reference {
    pub id: RecordId,
    pub title: String,
    pub description: String,
    pub url: String,
    pub tags: Vec<String>,
    pub tech_stack: Vec<String>,
    pub teck_stack_percentage: Vec<u8>,
    pub created_at: String,
    pub updated_at: String,
    pub is_published: bool,
    pub year: Option<String>,
    pub category: Option<String>,
    pub icon: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn post(created_at: &str, content_updated_at: Option<&str>) -> Post {
        Post {
            created_at: created_at.into(),
            content_updated_at: content_updated_at.map(Into::into),
            ..Post::default()
        }
    }

    #[test]
    fn unedited_and_same_day_posts_show_no_update() {
        assert_eq!(post("2024-10-01T09:00:00Z", None).updated_on(), None);
        assert_eq!(
            post("2024-10-01T09:00:00Z", Some("2024-10-01T18:30:00Z")).updated_on(),
            None
        );
    }

    #[test]
    fn later_edits_show_their_date() {
        let edited = post("2024-10-01T09:00:00Z", Some("2025-02-03T10:00:00.123456Z"));
        assert_eq!(edited.updated_on().as_deref(), Some("Feb 3, 2025"));
    }
}
