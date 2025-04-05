use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub fn normalize_symbol(input: &str) -> String {
    input.trim().to_lowercase()
}

pub fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
