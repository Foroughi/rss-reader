use crate::domain::item::Item;
use crate::sources::source::Source;

use futures::future::join_all;

pub struct Aggregator {
    sources: Vec<Box<dyn Source>>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn add_source<S: Source + 'static>(&mut self, source: S) {
        self.sources.push(Box::new(source));
    }

    pub async fn fetch_all(&self) -> Vec<Item> {
        let futures = self.sources.iter().map(|source| source.fetch());

        let results = join_all(futures).await;

        let mut items = Vec::new();

        for result in results {
            match result {
                Ok(mut fetched) => items.append(&mut fetched),
                Err(err) => {
                    eprintln!("Error fetching source: {:?}", err);
                }
            }
        }

        items
    }
}
