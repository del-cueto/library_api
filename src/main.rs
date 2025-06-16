mod config;
mod domain;
mod app;
mod infra;
mod handlers;

use axum::{
    Router,
    routing::{get, post, put, delete},
    serve,
};
use tokio::net::TcpListener;
use sqlx::sqlite::SqlitePoolOptions;
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber;

use crate::config::{load_env, database_url};
use crate::infra::sqlite_book_repository::SqliteBookRepository;
use crate::handlers::book_handler::{
    get_books, get_book, post_book,
    put_book, delete_book, search_books,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    load_env();

    let pool = SqlitePoolOptions::new()
        .connect(&database_url())
        .await
        .expect("Failed to connect to DB");
    let repo = Arc::new(SqliteBookRepository { pool });

    let app = Router::new()
        .route("/books", get(get_books).post(post_book))
        .route(
            "/books/:id",
            get(get_book)
                .put(put_book)
                .delete(delete_book),
        )
        .route("/books/search", get(search_books))
        .with_state(repo);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    // axum::serve reemplaza a hyper::Server en Axum 0.7 :contentReference[oaicite:0]{index=0}
    serve(listener, app).await.unwrap();
}
