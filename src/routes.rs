use crate::api::fetch_news;
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewsQuery {
    coin: String,
}

pub async fn get_news(Query(params): Query<NewsQuery>) -> Html<String> {
    let news = fetch_news(&params.coin)
        .await
        .unwrap_or("Error fetching data".to_string());
    Html(format!(
        r#"
        <html>
            <head><title>Crypto News</title><link rel="stylesheet" href="/static/style.css"></head>
            <body>
                <h1>{}</h1>
                <a href="/">Back</a>
            </body>
        </html>
        "#,
        news
    ))
}

pub fn create_router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { Html(std::fs::read_to_string("src/templates/index.html").unwrap()) }),
        )
        .route("/news", get(get_news))
}
