use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub id: Thing,
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
            id: Thing::from(("author", "0")),
            name: String::new(),
            email: String::new(),
            bio: None,
            linkedin: None,
            twitter: None,
            github: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub id: Thing,
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
    pub is_published: bool,
    pub header_image: Option<String>,
    pub show_cta: bool,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: Thing::from(("post", "0")),
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
            is_published: true,
            header_image: None,
            show_cta: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reference {
    pub id: Thing,
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
