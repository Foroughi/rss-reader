use crate::domain::item::Item;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rss: Vec<SourceConfig>,
}
#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub url: String,
    pub tag: Option<String>,
}

#[async_trait]
pub trait Source: Send + Sync {
    async fn fetch(&self) -> anyhow::Result<Vec<Item>>;
    fn tag(&self) -> &str;
    fn name(&self) -> &str;
}
