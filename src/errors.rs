use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("External API error: {0}")]
    ApiError(String),

    #[error("Redis error: {0}")]
    RedisError(#[from] deadpool_redis::redis::RedisError),

    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Unexpected error: {0}")]
    Other(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json({
            serde_json::json!({ "error": self.to_string() })
        })
    }
}
