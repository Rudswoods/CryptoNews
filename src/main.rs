mod api;  // Declare the api module
mod routes;  // Declare the routes module
mod cache;  // Add this line
mod auth;
mod db;

use axum::{
    Router,
    routing::{get, post},
    http::{HeaderValue, Method},
};
use std::sync::Arc;
use cache::RedisCache;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::sync::broadcast;

use crate::{
    auth::{login_page, register_page, handle_login, handle_register, NewsUpdate},
    routes::homepage,
    db::Database,
};

#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<RedisCache>,
    pub tx: broadcast::Sender<NewsUpdate>,
    pub db: Arc<Database>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    let cache = Arc::new(RedisCache::new());
    let (tx, _) = broadcast::channel(100);
    let db = Arc::new(Database::new().await.expect("Failed to initialize database"));
    
    let state = AppState {
        cache: cache.clone(),
        tx: tx.clone(),
        db: db.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("*"))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(homepage))
        .route("/login", get(login_page).post(handle_login))
        .route("/register", get(register_page).post(handle_register))
        .route("/ws", get(auth::handle_ws))
        .route("/search", get(routes::handle_search))
        .route("/search", post(routes::handle_search_post))
        .route("/stats", get(routes::cache_stats))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
