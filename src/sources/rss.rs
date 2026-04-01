use super::source::Source;
use crate::domain::item::{Item, SourceKind};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rss::Channel;

pub struct RssSource {
    pub url: String,
    pub tag: Option<String>, // optional tag like "rust", "dev"
}

impl RssSource {
    pub fn new(url: String) -> Self {
        Self { url, tag: None }
    }
}

#[async_trait]
impl Source for RssSource {
    async fn fetch(&self) -> anyhow::Result<Vec<Item>> {
        // fetch feed
        let content = reqwest::get(&self.url).await?.bytes().await?;

        // parse RSS
        let channel = Channel::read_from(&content[..])?;

        let mut items = Vec::new();

        for entry in channel.items() {
            let title = entry.title().unwrap_or("No title").to_string();
            let url = entry.link().unwrap_or("").to_string();

            if url.is_empty() {
                continue; // skip broken items
            }

            // fallback ID → use link
            let id = entry
                .guid()
                .map(|g| g.value().to_string())
                .unwrap_or_else(|| url.clone());

            // parse date (optional)
            let created_at = entry
                .pub_date()
                .and_then(|d| DateTime::parse_from_rfc2822(d).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);

            let mut item = Item::new(
                id,
                title,
                url,
                SourceKind::Rss,
                created_at,
            );

            // optional content
            item.content = entry.description().map(|d| d.to_string());

            // optional tag
            if let Some(tag) = &self.tag {
                item.tags.push(tag.clone());
            }

            items.push(item);
        }

        Ok(items)
    }

    fn name(&self) -> &str {
        "rss"
    }
}
