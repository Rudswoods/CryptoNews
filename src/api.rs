use dotenv::dotenv;
use reqwest;
use serde_json::Value;
use std::env;

pub async fn fetch_news(coin: &str) -> Result<String, reqwest::Error> {
    dotenv().ok();

    let base_url = env::var("BASE_URL").expect("BASE_URL must be set in .env");
    let api_key = env::var("API_KEY").expect("API_KEY must be set in .env");

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
        Ok(format!("Current price of {}: ${:.2}", coin, price))
    } else {
        Ok("Error: Could not retrieve price data.".to_string())
    }
}
