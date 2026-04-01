use crate::domain::item::Item;
use async_trait::async_trait;

#[async_trait]
pub trait Source: Send + Sync {
    async fn fetch(&self) -> anyhow::Result<Vec<Item>>;

    fn name(&self) -> &str;
}
