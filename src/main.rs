mod config;
mod domain;
mod app;
mod infra;
mod handlers;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use config::mod_config;
use infra::sqlite_book_repository::SqliteBookRepository;
use handlers::book_handler;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    mod_config::load_env();

    let db_url = mod_config::database_url();
    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    let repo = Arc::new(SqliteBookRepository { pool });

    let app = Router::new()
        .route("/books", get(book_handler::get_books))
        .with_state(repo);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
