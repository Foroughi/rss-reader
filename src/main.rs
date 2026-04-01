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
        url: "https://feeds.arstechnica.com/arstechnica/index".into(),
        tag: Some("Ars Technica".into()),
    });
    aggregator.add_source(RssSource {
        url: "http://rss.slashdot.org/Slashdot/slashdotMain".into(),
        tag: Some("Slashdot".into()),
    });
    aggregator.add_source(RssSource {
        url: "https://feeds.feedburner.com/TheHackersNews".into(),
        tag: Some("The Hacker News".into()),
    });
    aggregator.add_source(RssSource {
        url: "https://hnrss.org/frontpage".into(),
        tag: Some("Hacker news".into()),
    });

    let mut items = aggregator.fetch_all().await;
    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut ui = Ui::new(items);
    ui.run()?;

    Ok(())
}
