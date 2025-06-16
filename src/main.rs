mod config;
mod domain;
mod infra;
mod handlers;
mod middleware;
mod error;
mod app;

use axum::serve;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber;
use std::{sync::Arc, net::SocketAddr};
use crate::{config::{load_env, database_url}, infra::sqlite_book_repository::SqliteBookRepository, app::build_app};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    load_env();

    let pool = SqlitePoolOptions::new()
        .connect(&database_url())
        .await
        .expect("DB connect");
    let repo = Arc::new(SqliteBookRepository { pool });

    let app = build_app(repo);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server on http://{}", addr);

    let listener = TcpListener::bind(addr).await.expect("bind");
    serve(listener, app).await.expect("serve");
}
