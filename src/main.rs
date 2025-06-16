mod config;
mod domain;
mod app;
mod infra;
mod handlers;
mod middleware;
pub mod error;

use axum::{Router, routing::get, routing::post, middleware as axum_middleware};
use axum::routing::put;
use axum::serve;
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
use crate::handlers::auth_handler::login;
use crate::middleware::auth::auth;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    load_env();

    let pool = SqlitePoolOptions::new()
        .connect(&database_url())
        .await
        .expect("Failed to connect to DB");
    let repo = Arc::new(SqliteBookRepository { pool });

    // Rutas públicas
    let public = Router::new()
        .route("/login", post(login))
        .route("/books", get(get_books))
        .route("/books/:id", get(get_book))
        .route("/books/search", get(search_books))
        .with_state(repo.clone());

    // Rutas protegidas (POST, PUT, DELETE)
    let protected = Router::new()
        .route("/books", post(post_book))
        .route("/books/:id", put(put_book).delete(delete_book))
        .with_state(repo)
        .layer(axum_middleware::from_fn(auth));

    let app = public.merge(protected);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.expect("bind failed");
    serve(listener, app).await.unwrap();
}
