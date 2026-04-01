use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub url: String,
    pub source: SourceKind,
    pub created_at: DateTime<Utc>,
    pub score: Option<i32>,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub is_read: bool,
    pub content: Option<String>,
}

impl Item {
    pub fn new(
        id: String,
        title: String,
        url: String,
        source: SourceKind,
        // content : String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            url,
            source,
            created_at,
            content: None,
            score: None,
            author: None,
            tags: vec![],
            is_read: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SourceKind {
    Rss,
    Reddit,
    HackerNews,
    YouTube,
}
