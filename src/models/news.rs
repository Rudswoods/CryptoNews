struct NewsArticle {
    title: String,
    source: String,
    date: String,
    summary: String,
    link: String,
}

impl NewsArticle {
    pub fn new(title: String, source: String, date: String, summary: String, link: String) -> Self {
        NewsArticle {
            title,
            source,
            date,
            summary,
            link,
        }
    }
}