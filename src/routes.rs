use axum::{
    extract::{Form, Query, State},
    response::{Html, IntoResponse},
    http::StatusCode,
};
use serde::Deserialize;
use crate::{AppState, api, auth};

#[derive(Debug, Deserialize)]
pub struct SearchForm {
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn homepage(State(state): State<AppState>) -> Html<String> {
    let top_searches = state.cache.get_top_searches().await;
    
    let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Crypto News Search</title>
            <style>
                body {{ font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }}
                .nav-container {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 10px 20px;
                    background-color: #333;
                    color: white;
                    border-radius: 4px;
                    margin-bottom: 20px;
                }}
                .nav-title {{
                    font-size: 1.2em;
                    font-weight: bold;
                }}
                .nav-buttons {{
                    display: flex;
                    gap: 10px;
                }}
                .nav-button {{
                    padding: 8px 15px;
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    text-decoration: none;
                    font-size: 14px;
                }}
                .nav-button:hover {{ background-color: #45a049; }}
                .search-container {{ text-align: center; margin: 40px 0; }}
                input[type="text"] {{ 
                    width: 60%; 
                    padding: 10px; 
                    font-size: 16px; 
                    border: 2px solid #ddd; 
                    border-radius: 4px; 
                }}
                button {{ 
                    padding: 10px 20px; 
                    font-size: 16px; 
                    background-color: #4CAF50; 
                    color: white; 
                    border: none; 
                    border-radius: 4px; 
                    cursor: pointer; 
                }}
                button:hover {{ background-color: #45a049; }}
                .top-searches {{
                    margin-top: 20px;
                    padding: 20px;
                    background-color: #f9f9f9;
                    border-radius: 4px;
                }}
                .top-searches h2 {{ margin-top: 0; }}
                .search-item {{
                    margin: 10px 0;
                    padding: 10px;
                    background-color: white;
                    border-radius: 4px;
                    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
                }}
            </style>
        </head>
        <body>
            <div class="nav-container">
                <div class="nav-title">Crypto News</div>
                <div class="nav-buttons" id="authButtons">
                    <a href="/register" class="nav-button">Register</a>
                    <a href="/login" class="nav-button">Login</a>
                </div>
            </div>
            <div class="search-container">
                <h1>Crypto News Search</h1>
                <form action="/search" method="get">
                    <input type="text" name="q" placeholder="Enter cryptocurrency name or symbol..." required>
                    <button type="submit">Search</button>
                </form>
            </div>
            <div class="top-searches">
                <h2>Top Searches</h2>
                {}
            </div>
            <script>
                // Check authentication status
                const token = localStorage.getItem('token');
                const authButtons = document.getElementById('authButtons');
                
                if (token) {{
                    authButtons.innerHTML = `
                        <span class="nav-button" style="background-color: #666;">Welcome!</span>
                        <button class="nav-button" onclick="logout()">Logout</button>
                    `;
                }}
                
                function logout() {{
                    localStorage.removeItem('token');
                    window.location.reload();
                }}
            </script>
        </body>
        </html>
    "#, 
    if top_searches.is_empty() {
        "<p>No searches yet</p>".to_string()
    } else {
        top_searches.iter()
            .map(|(term, count)| {
                format!(
                    r#"<div class="search-item">{} - {} searches</div>"#,
                    term, count
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    });

    Html(html)
}

pub async fn handle_search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Html<String>, StatusCode> {
    let cache_key = format!("news:{}", query.q);
    
    // Try to get from cache first
    if let Some(cached_html) = state.cache.get(&cache_key).await {
        state.cache.increment_search_count(&query.q).await;
        return Ok(Html(cached_html));
    }

    // If not in cache, fetch from API
    match api::fetch_news(&query.q).await {
        Ok(news) => {
            let html = auth::format_news_html(&query.q, &news);
            state.cache.set(&cache_key, &html).await;
            state.cache.increment_search_count(&query.q).await;
            Ok(Html(html))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn handle_search_post(
    State(state): State<AppState>,
    axum::extract::Form(query): axum::extract::Form<SearchQuery>,
) -> Result<Html<String>, StatusCode> {
    handle_search(State(state), Query(query)).await
}

pub async fn cache_stats(State(state): State<AppState>) -> Html<String> {
    let stats = state.cache.get_stats().await;
    
    let html = format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Cache Statistics</title>
            <style>
                body {{ font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }}
                .nav-container {{
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 10px 20px;
                    background-color: #333;
                    color: white;
                    border-radius: 4px;
                    margin-bottom: 20px;
                }}
                .nav-title {{
                    font-size: 1.2em;
                    font-weight: bold;
                }}
                .nav-buttons {{
                    display: flex;
                    gap: 10px;
                }}
                .nav-button {{
                    padding: 8px 15px;
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    cursor: pointer;
                    text-decoration: none;
                    font-size: 14px;
                }}
                .nav-button:hover {{ background-color: #45a049; }}
                .stats-container {{ 
                    background-color: #f9f9f9; 
                    padding: 20px; 
                    border-radius: 4px; 
                    margin-top: 20px; 
                }}
                .stat-item {{ 
                    margin: 10px 0; 
                    padding: 10px; 
                    background-color: white; 
                    border-radius: 4px; 
                    box-shadow: 0 1px 3px rgba(0,0,0,0.1); 
                }}
            </style>
        </head>
        <body>
            <div class="nav-container">
                <div class="nav-title">Crypto News</div>
                <div class="nav-buttons">
                    <a href="/register" class="nav-button">Register</a>
                    <a href="/login" class="nav-button">Login</a>
                </div>
            </div>
            <h1>Cache Statistics</h1>
            <div class="stats-container">
                <div class="stat-item">
                    <strong>Total Keys:</strong> {}
                </div>
                <div class="stat-item">
                    <strong>Total Memory Used:</strong> {:.2} MB
                </div>
                <div class="stat-item">
                    <strong>Hit Rate:</strong> {:.2}%
                </div>
            </div>
            <br>
            <a href="/" style="color: #4CAF50; text-decoration: none;">Back to Homepage</a>
        </body>
        </html>
    "#, 
        stats.total_keys,
        stats.memory_used as f64 / 1024.0 / 1024.0,
        stats.hit_rate * 100.0
    );

    Html(html)
}
