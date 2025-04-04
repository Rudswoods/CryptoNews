use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub published_at: String,
    pub summary: String,
    pub url: String,
}

pub async fn fetch_price(coin: &str) -> Result<String, reqwest::Error> {
    dotenv().ok();

    let base_url = env::var("COINMARKETCAP_URL").expect("COINMARKETCAP_URL must be set in .env");
    let api_key =
        env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY must be set in .env");

    let url = format!("{}?symbol={}&convert=USD", base_url, coin.to_uppercase());
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&response).unwrap_or_else(|_| Value::Null);

    if let Some(price) = json["data"][coin.to_uppercase()]["quote"]["USD"]["price"].as_f64() {
        Ok(format!("${:.2}", price))
    } else {
        Ok("Price data unavailable".to_string())
    }
}

pub async fn fetch_news(coin: &str) -> Result<Vec<NewsItem>, reqwest::Error> {
    dotenv().ok();

    let news_url =
        env::var("CRYPTO_NEWS_API_URL").expect("CRYPTO_NEWS_API_URL must be set in .env");

    let news_api_key =
        env::var("CRYPTO_NEWS_API_KEY").expect("CRYPTO_NEWS_API_KEY must be set in .env");

    let url = format!(
        "{}?tickers={}&items=3&token={}",
        news_url,
        coin.to_uppercase(),
        news_api_key
    );

    println!(
        "Requesting news from: {}",
        url.replace(&news_api_key, "[API_KEY]")
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?.text().await?;

    println!(
        "Response preview: {}",
        &response[..std::cmp::min(200, response.len())]
    );

    let json: Value = serde_json::from_str(&response).unwrap_or_else(|_| {
        println!("Failed to parse JSON response");
        Value::Null
    });

    let mut news_items = Vec::new();

    if let Some(message) = json["message"].as_str() {
        println!("API returned message: {}", message);
    }

    if let Some(data) = json["data"].as_array() {
        println!("Found {} news items in response", data.len());

        for item in data.iter() {
            let title = item["title"].as_str();
            let source = item["source_name"].as_str();
            let date = item["date"].as_str();
            let text = item["text"].as_str();
            let url = item["news_url"].as_str();

            if let (Some(title), Some(source), Some(date), Some(text), Some(url)) =
                (title, source, date, text, url)
            {
                news_items.push(NewsItem {
                    title: title.to_string(),
                    source: source.to_string(),
                    published_at: date.to_string(),
                    summary: if text.len() > 150 {
                        format!("{}...", &text[0..150])
                    } else {
                        text.to_string()
                    },
                    url: url.to_string(),
                });
            }
        }
    } else {
        println!("No 'data' array found in API response");

        if let Some(news) = json["news"].as_array() {
            println!("Found alternative 'news' array with {} items", news.len());

            for item in news.iter() {
                let title = item["title"].as_str();
                let source = item["source"]
                    .as_str()
                    .or_else(|| item["source_name"].as_str());
                let date = item["published_at"]
                    .as_str()
                    .or_else(|| item["date"].as_str());
                let text = item["description"]
                    .as_str()
                    .or_else(|| item["text"].as_str());
                let url = item["url"].as_str().or_else(|| item["news_url"].as_str());

                if let (Some(title), Some(source), Some(date), Some(text), Some(url)) =
                    (title, source, date, text, url)
                {
                    news_items.push(NewsItem {
                        title: title.to_string(),
                        source: source.to_string(),
                        published_at: date.to_string(),
                        summary: if text.len() > 150 {
                            format!("{}...", &text[0..150])
                        } else {
                            text.to_string()
                        },
                        url: url.to_string(),
                    });
                }
            }
        }
    }

    println!(
        "Found {} relevant news items for {}",
        news_items.len(),
        coin
    );
    Ok(news_items)
}
