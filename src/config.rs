use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub redis_url: String,
    pub database_url: String,
    pub cryptqnews_api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            cryptqnews_api_key: env::var("CRYPTQNEWS_API_KEY").expect("CRYPTQNEWS_API_KEY must be set"),
        }
    }
}
