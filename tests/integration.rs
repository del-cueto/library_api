use tokio::task;
use reqwest::StatusCode;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::sync::Arc;
use once_cell::sync::Lazy;
use serde_json::json;
use library_api::app::build_app;
use library_api::infra::sqlite_book_repository::SqliteBookRepository;
use axum::serve;
use tokio::net::TcpListener;

// BD en memoria compartida para los tests
static DB_URL: Lazy<String> = Lazy::new(|| "sqlite::memory:".to_string());

async fn spawn_app() -> String {
    // obtener token
    std::env::set_var("JWT_SECRET", "test-secret");
    // Conectar y migrar la BD en memoria
    
    let pool: SqlitePool = SqlitePoolOptions::new()
        .connect(&DB_URL)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // Construir el repo y la app
    let repo = Arc::new(SqliteBookRepository { pool });
    let app = build_app(repo);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    task::spawn(async move {
        serve(listener, app).await.unwrap();
    });

    format!("http://{}", addr)
}

async fn get_token(base: &str) -> String {
    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/login", base))
        .json(&json!({ "username": "admin", "password": "password" }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    res.json::<String>().await.unwrap()
}

#[tokio::test]
async fn post_and_get_book_flow() {
    let base = spawn_app().await;
    let client = reqwest::Client::new();
    let token = get_token(&base).await;

    // 1) Crear libro
    let create_res = client
        .post(&format!("{}/books", &base))
        .bearer_auth(&token)
        .json(&json!({
            "title": "The Hobbit",
            "author": "J.R.R. Tolkien",
            "published_year": 1937
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(create_res.status(), StatusCode::CREATED);
    let created: serde_json::Value = create_res.json().await.unwrap();
    let id = created["id"].as_str().unwrap();

    // 2) GET /books should contain the new book
    let list_res = client
        .get(&format!("{}/books", &base))
        .send()
        .await
        .unwrap();
    assert_eq!(list_res.status(), StatusCode::OK);
    let list: Vec<serde_json::Value> = list_res.json().await.unwrap();
    assert!(list.iter().any(|b| b["id"] == id));

    // 3) GET /books/:id returns that book
    let get_res = client
        .get(&format!("{}/books/{}", &base, id))
        .send()
        .await
        .unwrap();
    assert_eq!(get_res.status(), StatusCode::OK);
    let fetched: serde_json::Value = get_res.json().await.unwrap();
    assert_eq!(fetched["title"], "The Hobbit");
}

#[tokio::test]
async fn put_and_delete_book_flow() {
    let base = spawn_app().await;
    let client = reqwest::Client::new();
    let token = get_token(&base).await;

    // Setup: crear un libro
    let created: serde_json::Value = client
        .post(&format!("{}/books", &base))
        .bearer_auth(&token)
        .json(&json!({
            "title": "1984",
            "author": "George Orwell",
            "published_year": 1949
        }))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let id = created["id"].as_str().unwrap();

    // 1) PUT /books/:id
    let put_res = client
        .put(&format!("{}/books/{}", &base, id))
        .bearer_auth(&token)
        .json(&json!({ "title": "Nineteen Eighty-Four" }))
        .send()
        .await
        .unwrap();
    assert_eq!(put_res.status(), StatusCode::OK);
    let updated: serde_json::Value = put_res.json().await.unwrap();
    assert_eq!(updated["title"], "Nineteen Eighty-Four");

    // 2) DELETE /books/:id
    let del_res = client
        .delete(&format!("{}/books/{}", &base, id))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert_eq!(del_res.status(), StatusCode::NO_CONTENT);

    // 3) GET /books/:id ahora 404
    let not_found = client
        .get(&format!("{}/books/{}", &base, id))
        .send()
        .await
        .unwrap();
    assert_eq!(not_found.status(), StatusCode::NOT_FOUND);
}
#[tokio::test]
async fn get_books_returns_empty_list() {
    let base = spawn_app().await;
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/books", &base))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert!(body.is_empty());
}

#[tokio::test]
async fn post_book_without_auth_is_unauthorized() {
    let base = spawn_app().await;
    let client = reqwest::Client::new();

    let resp = client
        .post(&format!("{}/books", &base))
        .json(&serde_json::json!({
            "title": "foo",
            "author": "bar"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["error"], "Unauthorized");
}
