// This file contains functions to interact with the CoinGecko API, retrieving cryptocurrency data and news.

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct CoinGeckoResponse {
    articles: Vec<Article>,
}

#[derive(Deserialize)]
struct Article {
    title: String,
    source: String,
    published_at: String,
    summary: String,
    url: String,
}

pub async fn fetch_news(crypto: &str) -> Result<Vec<Article>, Error> {
    let url = format!("https://api.coingecko.com/api/v3/news?query={}", crypto);
    let response: CoinGeckoResponse = reqwest::get(&url).await?.json().await?;
    Ok(response.articles)
}