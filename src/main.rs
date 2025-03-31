// main.rs
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::Mutex;
use crate::api::cryptonews::fetch_news;
use crate::models::news::NewsRequest;

#[macro_use]
extern crate serde_derive;

mod api;
mod models;
mod services;
mod web;

struct AppState {
    // Add any shared state here, e.g., cache
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(Mutex::new(AppState {}));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(web::index))
            .route("/news", web::post().to(get_news))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_news(req: web::Json<NewsRequest>) -> impl Responder {
    match fetch_news(&req.symbol).await {
        Ok(news) => HttpResponse::Ok().json(news),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}