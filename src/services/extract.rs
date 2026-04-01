use scraper::{Html, Selector};

pub fn extract_readable(html: &str) -> String {
    let doc = Html::parse_document(html);

    // 1. Try <article> first (best case)
    let article_sel = Selector::parse("article").unwrap();

    if let Some(article) = doc.select(&article_sel).next() {
        let text = article
            .text()
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        if !text.is_empty() {
            return text;
        }
    }

    // 2. Fallback: collect all <p>
    let p_sel = Selector::parse("p").unwrap();

    let text = doc
        .select(&p_sel)
        .map(|p| {
            p.text()
                .map(|t| t.trim())
                .filter(|t| !t.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");

    text
}
