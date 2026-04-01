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
        let content = reqwest::get(&self.url).await?.bytes().await?;

        // try RSS first
        let channel = Channel::read_from(&content[..]);

        let items = if let Ok(channel) = channel {
            channel.items().iter().map(|entry| {
                // same mapping as before
                let title = entry.title().unwrap_or("No title").to_string();
                let url = entry.link().unwrap_or("").to_string();

                let id = entry.guid()
                    .map(|g| g.value().to_string())
                    .unwrap_or_else(|| url.clone());

                let created_at = entry.pub_date()
                    .and_then(|d| chrono::DateTime::parse_from_rfc2822(d).ok())
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(chrono::Utc::now);

                let mut item = Item::new(id, title, url, SourceKind::Rss, created_at);

                // item.content = entry.description().map(|d| d.to_string());

                item
            }).collect()

        } else {
            // fallback to Atom
            let feed = atom_syndication::Feed::read_from(&content[..])?;

            feed.entries().iter().map(|entry| {
                let title = entry.title().to_string();
                let url = entry.links().get(0)
                    .map(|l| l.href.clone())
                    .unwrap_or_default();

                let id = entry.id().to_string();

                let created_at = entry.published()
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .unwrap_or_else(chrono::Utc::now);

                let mut item = Item::new(id, title, url, SourceKind::Rss, created_at);

                // item.content = entry.summary().map(|s| s.to_string());

                item
            }).collect()
        };  
        Ok(items)
    }

    fn name(&self) -> &str {
        "rss"
    }
}
