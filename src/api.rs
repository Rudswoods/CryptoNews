use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub url: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date: DateTime<Utc>,
    pub summary: String,
    pub sentiment: f32,  // -1.0 to 1.0, where -1 is negative, 0 is neutral, 1 is positive
}

// Enhanced sentiment analysis function
fn calculate_sentiment(text: &str) -> f32 {
    let positive_words = vec![
        ("bullish", 0.5),
        ("surge", 0.4),
        ("rise", 0.3),
        ("gain", 0.3),
        ("growth", 0.3),
        ("up", 0.2),
        ("positive", 0.3),
        ("success", 0.4),
        ("breakthrough", 0.5),
        ("adoption", 0.4),
        ("partnership", 0.3),
        ("development", 0.2),
        ("innovation", 0.3),
        ("upgrade", 0.3),
        ("launch", 0.3),
        ("milestone", 0.4),
        ("record", 0.3),
        ("increase", 0.3),
        ("expansion", 0.3),
        ("investment", 0.2)
    ];
    
    let negative_words = vec![
        ("bearish", -0.5),
        ("crash", -0.5),
        ("fall", -0.4),
        ("drop", -0.4),
        ("decline", -0.3),
        ("down", -0.2),
        ("negative", -0.3),
        ("fail", -0.4),
        ("risk", -0.3),
        ("loss", -0.4),
        ("concern", -0.2),
        ("warning", -0.3),
        ("threat", -0.4),
        ("weak", -0.3),
        ("ban", -0.5),
        ("hack", -0.5),
        ("scam", -0.5),
        ("fraud", -0.5),
        ("regulation", -0.2),
        ("restriction", -0.3)
    ];
    
    let text = text.to_lowercase();
    let mut sentiment: f32 = 0.0;
    let mut word_count = 0;
    
    for (word, weight) in positive_words {
        if text.contains(word) {
            sentiment += weight;
            word_count += 1;
        }
    }
    
    for (word, weight) in negative_words {
        if text.contains(word) {
            sentiment += weight;
            word_count += 1;
        }
    }
    
    // Normalize sentiment based on word count
    if word_count > 0 {
        sentiment = sentiment / (word_count as f32);
    }
    
    // Clamp sentiment between -1.0 and 1.0
    if sentiment > 1.0 {
        1.0
    } else if sentiment < -1.0 {
        -1.0
    } else {
        sentiment
    }
}

pub async fn fetch_news(coin: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    
    let api_key = env::var("CRYPTO_NEWS_API_KEY").expect("CRYPTO_NEWS_API_KEY must be set");
    let api_url = env::var("CRYPTO_NEWS_API_URL").expect("CRYPTO_NEWS_API_URL must be set");
    
    let url = format!("{}/news?tickers={}&items=50&page=1", api_url, coin);
    let client = reqwest::Client::new();
    
    let response = client
        .get(&url)
        .header("x-api-key", api_key)
        .send()
        .await?;

    let news_data: serde_json::Value = response.json().await?;
    
    let mut news_items = Vec::new();
    
    if let Some(data) = news_data.get("data") {
        if let Some(items) = data.as_array() {
            for item in items {
                if let (Some(title), Some(source), Some(url), Some(date), Some(summary)) = (
                    item.get("title").and_then(|t| t.as_str()),
                    item.get("source").and_then(|s| s.as_str()),
                    item.get("news_url").and_then(|u| u.as_str()),
                    item.get("date").and_then(|d| d.as_str()),
                    item.get("text").and_then(|t| t.as_str()),
                ) {
                    // Parse date
                    let date = DateTime::parse_from_rfc3339(date)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());

                    // Calculate sentiment using both title and summary
                    let title_sentiment = calculate_sentiment(title);
                    let summary_sentiment = calculate_sentiment(summary);
                    let sentiment = (title_sentiment + summary_sentiment) / 2.0;

                    news_items.push(NewsItem {
                        title: title.to_string(),
                        source: source.to_string(),
                        url: url.to_string(),
                        date,
                        summary: summary.to_string(),
                        sentiment,
                    });
                }
            }
        }
    }

    Ok(news_items)
}

