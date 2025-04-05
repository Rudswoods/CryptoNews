use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsArticle {
    pub id: Uuid,
    pub title: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
    pub summary: String,
    pub url: String,
    pub symbol: String,
    pub cached_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub redis_client: deadpool_redis::Pool,
    pub db_pool: sqlx::PgPool,
}
