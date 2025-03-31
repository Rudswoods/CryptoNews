// This file contains functions to interact with the CryptQNews API, fetching the latest news articles based on user input.

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct NewsArticle {
    title: String,
    source: String,
    date: String,
    summary: String,
    link: String,
}

pub async fn fetch_latest_news(crypto: &str) -> Result<Vec<NewsArticle>, Error> {
    let url = format!("https://api.cryptqnews.com/v1/news?crypto={}", crypto);
    let response = reqwest::get(&url).await?;
    let articles: Vec<NewsArticle> = response.json().await?;
    Ok(articles)
}