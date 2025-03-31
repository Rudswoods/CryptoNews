use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Cache {
    data: HashMap<String, (String, Instant)>, // Stores news articles with their expiration time
    ttl: Duration, // Time to live for cached items
}

impl Cache {
    pub fn new(ttl: Duration) -> Self {
        Cache {
            data: HashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        if let Some((value, timestamp)) = self.data.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value);
            }
        }
        None
    }

    pub fn set(&mut self, key: String, value: String) {
        let timestamp = Instant::now();
        self.data.insert(key, (value, timestamp));
    }

    pub fn clear(&mut self) {
        self.data.retain(|_, (_, timestamp)| timestamp.elapsed() < self.ttl);
    }
}