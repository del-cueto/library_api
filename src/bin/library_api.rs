use library_api::{ config::load_env,
                   infra::sqlite_book_repository::SqliteBookRepository,
                   app::build_app };
use axum::serve;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use std::{sync::Arc, net::SocketAddr};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    load_env();

    let pool = SqlitePoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let repo = Arc::new(SqliteBookRepository { pool });
    let app  = build_app(repo);

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("🚀 http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
