mod app;
mod config;
mod domain;
mod services;
mod sources;
mod ui;

use services::aggregator::Aggregator;
use sources::rss::RssSource;
use ui::ui::Ui;

use crate::config::load_config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config()?;
    let mut aggregator = Aggregator::new();
    for source in config.rss {
        aggregator.add_source(RssSource {
            url: source.url,
            tag: source.tag,
        });
    }

    let mut items = aggregator.fetch_all().await;
    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut ui = Ui::new(items);
    ui.run()?;

    Ok(())
}
