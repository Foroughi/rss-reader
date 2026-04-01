pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.text().await
}
