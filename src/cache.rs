use redis::Client;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct RedisCache {
    client: Client,
    hits: Arc<AtomicU64>,
    misses: Arc<AtomicU64>,
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_keys: usize,
    pub memory_used: usize,
    pub hit_rate: f64,
}

impl RedisCache {
    pub fn new() -> Self {
        let client = Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
        RedisCache {
            client,
            hits: Arc::new(AtomicU64::new(0)),
            misses: Arc::new(AtomicU64::new(0)),
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let mut con = self.client.get_async_connection().await.ok()?;
        match redis::cmd("GET").arg(key).query_async::<_, Option<String>>(&mut con).await {
            Ok(Some(value)) => {
                self.hits.fetch_add(1, Ordering::SeqCst);
                Some(value)
            }
            Ok(None) => {
                self.misses.fetch_add(1, Ordering::SeqCst);
                None
            }
            Err(_) => {
                self.misses.fetch_add(1, Ordering::SeqCst);
                None
            }
        }
    }

    pub async fn set(&self, key: &str, value: &str) -> bool {
        let mut con = match self.client.get_async_connection().await {
            Ok(con) => con,
            Err(_) => return false,
        };

        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(3600) // 1 hour expiration
            .query_async::<_, ()>(&mut con)
            .await
            .is_ok()
    }

    pub async fn increment_search_count(&self, term: &str) {
        let mut con = match self.client.get_async_connection().await {
            Ok(con) => con,
            Err(_) => return,
        };

        let _: Result<(), redis::RedisError> = redis::cmd("ZINCRBY")
            .arg("search_counts")
            .arg(1)
            .arg(term)
            .query_async(&mut con)
            .await;
    }

    pub async fn get_top_searches(&self) -> Vec<(String, u64)> {
        let mut con = match self.client.get_async_connection().await {
            Ok(con) => con,
            Err(_) => return Vec::new(),
        };

        match redis::cmd("ZREVRANGE")
            .arg("search_counts")
            .arg(0)
            .arg(4)
            .arg("WITHSCORES")
            .query_async::<_, Vec<(String, u64)>>(&mut con)
            .await
        {
            Ok(results) => results,
            Err(_) => Vec::new(),
        }
    }

    pub async fn get_stats(&self) -> CacheStats {
        let mut con = match self.client.get_async_connection().await {
            Ok(con) => con,
            Err(_) => return CacheStats {
                total_keys: 0,
                memory_used: 0,
                hit_rate: 0.0,
            },
        };

        let total_keys: usize = redis::cmd("DBSIZE")
            .query_async::<_, usize>(&mut con)
            .await
            .unwrap_or(0);

        let info: HashMap<String, String> = redis::cmd("INFO")
            .arg("memory")
            .query_async::<_, HashMap<String, String>>(&mut con)
            .await
            .unwrap_or_default();

        let memory_used = info
            .get("used_memory")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let hits = self.hits.load(Ordering::SeqCst);
        let misses = self.misses.load(Ordering::SeqCst);
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };

        CacheStats {
            total_keys,
            memory_used,
            hit_rate,
        }
    }
}
