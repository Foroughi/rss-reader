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
            channel
                .items()
                .iter()
                .map(|entry| {
                    let title = entry.title().unwrap_or("No title").to_string();
                    let url = entry.link().unwrap_or("").to_string();

                    let id = entry
                        .guid()
                        .map(|g| g.value().to_string())
                        .unwrap_or_else(|| url.clone());

                    let created_at = entry
                        .pub_date()
                        .and_then(|d| chrono::DateTime::parse_from_rfc2822(d).ok())
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now);

                    let mut item = Item::new(id, title, url, SourceKind::Rss, created_at);

                    // ✅ improved RSS content extraction
                    let content = entry
                        .content()
                        .filter(|c| !c.trim().is_empty())
                        .map(|c| c.to_string())
                        .or_else(|| {
                            entry
                                .description()
                                .filter(|d| !d.trim().is_empty())
                                .map(|d| d.to_string())
                        });

                    item.content = content.map(|html| html2text::from_read(html.as_bytes(), 80));

                    item
                })
                .collect()
        } else {
            // ✅ Atom fallback (FIXED)
            let feed = atom_syndication::Feed::read_from(&content[..])?;

            feed.entries()
                .iter()
                .map(|entry| {
                    let title = entry.title().to_string();
                    let url = entry
                        .links()
                        .get(0)
                        .map(|l| l.href.clone())
                        .unwrap_or_default();

                    let id = entry.id().to_string();

                    let created_at = entry
                        .published()
                        .map(|d| d.with_timezone(&chrono::Utc))
                        .unwrap_or_else(chrono::Utc::now);

                    let mut item = Item::new(id, title, url, SourceKind::Rss, created_at);

                    // 🔥 THIS IS THE IMPORTANT FIX
                    let content = entry
                        .content()
                        .and_then(|c| c.value.clone())
                        .filter(|c| !c.trim().is_empty())
                        .or_else(|| {
                            entry
                                .summary()
                                .map(|s| s.to_string())
                                .filter(|s| !s.trim().is_empty())
                        });

                    // item.content = content;
                    item.content = content.map(|html| html2text::from_read(html.as_bytes(), 80));
                    item
                })
                .collect()
        };

        Ok(items)
    }

    fn name(&self) -> &str {
        "rss"
    }

    fn tag(&self) -> &str {
        self.tag.as_deref().unwrap_or("")
    }
}
