#[derive(Clone, Debug)]
pub enum FeedType {
    Rss,
    HackerNews,
    Reddit,
    Youtube,
}

#[derive(Clone, Debug)]
pub struct Feed {
    pub name: String,
    pub url: String,
    pub feed_type: FeedType,
}
