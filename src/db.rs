use sqlx::{SqlitePool, Row};
use chrono::{NaiveDateTime, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub published_at: NaiveDateTime,
    pub summary: String,
    pub url: String,
}

pub async fn get_news(pool: &SqlitePool) -> Result<Vec<NewsItem>, sqlx::Error> {
    let rows = sqlx::query("SELECT title, source, published_at, summary, url FROM news_items")
        .fetch_all(pool)
        .await?;

    let news_items: Vec<NewsItem> = rows
        .into_iter()
        .map(|row| NewsItem {
            title: row.try_get("title").unwrap(),
            source: row.try_get("source").unwrap(),
            published_at: row.try_get::<String, _>("published_at")
                .unwrap()
                .parse::<NaiveDateTime>()
                .unwrap(),
            summary: row.try_get("summary").unwrap(),
            url: row.try_get("url").unwrap(),
        })
        .collect();

    Ok(news_items)
}

pub async fn create_news_item(pool: &SqlitePool, news_item: NewsItem) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO news_items (title, source, published_at, summary, url) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&news_item.title)
    .bind(&news_item.source)
    .bind(news_item.published_at.to_string())
    .bind(&news_item.summary)
    .bind(&news_item.url)
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Create data directory if it doesn't exist
        let data_dir = Path::new("data");
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).expect("Failed to create data directory");
        }
        
        // Create database file if it doesn't exist
        let database_path = data_dir.join("users.db");
        if !database_path.exists() {
            fs::File::create(&database_path).expect("Failed to create database file");
        }
        
        let database_url = format!("sqlite:{}", database_path.display());
        println!("Connecting to database at: {}", database_url);
        
        let pool = SqlitePool::connect(&database_url).await?;
        
        // Create users table if it doesn't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        // Create news_items table if it doesn't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS news_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                source TEXT NOT NULL,
                published_at TEXT NOT NULL,
                summary TEXT NOT NULL,
                url TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Database { pool })
    }

    pub async fn create_user(&self, username: &str, email: &str, password: &str) -> Result<User, String> {
        // Check if username or email already exists
        if let Ok(_) = self.get_user_by_username(username).await {
            return Err("Username already exists".to_string());
        }
        if let Ok(_) = self.get_user_by_email(email).await {
            return Err("Email already exists".to_string());
        }

        // Hash password
        let password_hash = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|e| format!("Failed to hash password: {}", e))?;

        // Insert new user
        let user = sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES (?, ?, ?)
            RETURNING id, username, email, password_hash
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(&password_hash)
        .map(|row: sqlx::sqlite::SqliteRow| User {
            id: row.try_get("id").unwrap(),
            username: row.try_get("username").unwrap(),
            email: row.try_get("email").unwrap(),
            password_hash: row.try_get("password_hash").unwrap(),
        })
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

        Ok(user)
    }

    pub async fn verify_user(&self, username: &str, password: &str) -> Result<User, String> {
        let user = self.get_user_by_username(username).await
            .map_err(|_| "User not found".to_string())?;

        if verify(password, &user.password_hash)
            .map_err(|e| format!("Failed to verify password: {}", e))? {
            Ok(user)
        } else {
            Err("Invalid password".to_string())
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<User, sqlx::Error> {
        sqlx::query(
            r#"
            SELECT id, username, email, password_hash
            FROM users
            WHERE username = ?
            "#,
        )
        .bind(username)
        .map(|row: sqlx::sqlite::SqliteRow| User {
            id: row.try_get("id").unwrap(),
            username: row.try_get("username").unwrap(),
            email: row.try_get("email").unwrap(),
            password_hash: row.try_get("password_hash").unwrap(),
        })
        .fetch_one(&self.pool)
        .await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query(
            r#"
            SELECT id, username, email, password_hash
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .map(|row: sqlx::sqlite::SqliteRow| User {
            id: row.try_get("id").unwrap(),
            username: row.try_get("username").unwrap(),
            email: row.try_get("email").unwrap(),
            password_hash: row.try_get("password_hash").unwrap(),
        })
        .fetch_one(&self.pool)
        .await
    }
}
