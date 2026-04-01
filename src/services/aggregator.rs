use crate::domain::item::Item;
use crate::sources::source::Source;
use futures::future::join_all;
use std::collections::HashMap;

pub struct Aggregator {
    pub sources: Vec<Box<dyn Source>>,
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
        let futures = self.sources.iter().map(|s| s.fetch());

        let results = futures::future::join_all(futures).await;

        let mut map: HashMap<String, Item> = HashMap::new();

        for (source, result) in self.sources.iter().zip(results) {
            match result {
                Ok(items) => {
                    for item in items {
                        let mut item = item;

                        item.tags.push(source.tag().to_string());

                        let key = item.url.clone();

                        map.entry(key)
                            .and_modify(|existing| {
                                if let (Some(a), Some(b)) = (existing.score, item.score) {
                                    existing.score = Some(a + b);
                                }

                                for tag in &item.tags {
                                    if !existing.tags.contains(tag) {
                                        existing.tags.push(tag.clone());
                                    }
                                }
                            })
                            .or_insert(item);
                    }
                }
                Err(err) => {
                    eprintln!("Error fetching source: {:?}", err);
                }
            }
        }

        let mut items: Vec<Item> = map.into_values().collect();

        // 🔥 ranking (simple but effective)
        items.sort_by(|a, b| {
            let score_a = a.score.unwrap_or(0);
            let score_b = b.score.unwrap_or(0);

            score_b
                .cmp(&score_a)
                .then_with(|| b.created_at.cmp(&a.created_at))
        });

        items
    }
}
