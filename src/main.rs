mod api;
mod routes;

use axum::Router;
use routes::create_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app: Router = create_router();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
