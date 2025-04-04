use crate::api::{fetch_news, fetch_price};
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewsQuery {
    coin: String,
}

pub async fn get_crypto_data(Query(params): Query<NewsQuery>) -> Html<String> {
    let coin = &params.coin;

    let price_future = fetch_price(coin);
    let news_future = fetch_news(coin);

    let (price_result, news_result) = tokio::join!(price_future, news_future);

    let price = price_result.unwrap_or_else(|_| "Error fetching price".to_string());
    let news_items = news_result.unwrap_or_else(|_| Vec::new());

    let news_html = if news_items.is_empty() {
        "<p>No news available for this cryptocurrency.</p>".to_string()
    } else {
        news_items
            .iter()
            .map(|item| {
                format!(
                    r#"
                <div class="news-item">
                    <h3 class="news-title"><a href="{}" target="_blank">{}</a></h3>
                    <div class="news-meta">
                        <span class="news-source">{}</span>
                        <span class="news-date">{}</span>
                    </div>
                    <p class="news-summary">{}</p>
                </div>
                "#,
                    item.url, item.title, item.source, item.published_at, item.summary
                )
            })
            .collect::<Vec<String>>()
            .join("")
    };

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Crypto News for {}</title>
                <link rel="stylesheet" href="/static/style.css">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
            </head>
            <body>
                <div class="container">
                    <h1>Crypto Data: {}</h1>
                    <div class="price-section">
                        <h2>Current Price: {}</h2>
                    </div>
                    
                    <div class="news-section">
                        <h2>Latest News</h2>
                        <div class="news-container">
                            {}
                        </div>
                    </div>
                    
                    <div class="back-link">
                        <a href="/">Back to Search</a>
                    </div>
                </div>
            </body>
        </html>
        "#,
        coin.to_uppercase(),
        coin.to_uppercase(),
        price,
        news_html
    ))
}

pub fn create_router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { Html(std::fs::read_to_string("src/templates/index.html").unwrap()) }),
        )
        .route("/news", get(get_crypto_data))
        .nest_service("/static", tower_http::services::ServeDir::new("src/static"))
}
