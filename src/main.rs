mod app;
mod config;
mod domain;
mod services;
mod sources;
mod ui;


use services::aggregator::Aggregator;
use sources::rss::RssSource;
use ui::ui::Ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut aggregator = Aggregator::new();

    aggregator.add_source(RssSource {
        url: "https://blog.rust-lang.org/feed.xml".into(),
        tag: Some("rust".into()),
    });

    aggregator.add_source(RssSource {
        url: "https://hnrss.org/frontpage".into(),
        tag: Some("hn".into()),
    });

    let mut items = aggregator.fetch_all().await;
    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut ui = Ui::new(items);
    ui.run()?;

    Ok(())
}
