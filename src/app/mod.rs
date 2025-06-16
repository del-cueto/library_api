pub mod book_repository;

use axum::{
    Router,
    routing::{get, post, put, delete},
    middleware::from_fn,
};
use std::sync::Arc;

use crate::{
    handlers::{
        book_handler::{
            get_books, get_book, post_book,
            put_book, delete_book, search_books,
        },
        auth_handler::login,
    },
    infra::sqlite_book_repository::SqliteBookRepository,
    middleware::auth::auth,
};

/// Construye el Router con rutas públicas y protegidas
pub fn build_app(repo: Arc<SqliteBookRepository>) -> Router {
    let public = Router::new()
        .route("/login", post(login))
        .route("/books", get(get_books))
        .route("/books/:id", get(get_book))
        .route("/books/search", get(search_books))
        .with_state(repo.clone());

    let protected = Router::new()
        .route("/books", post(post_book))
        .route("/books/:id", put(put_book).delete(delete_book))
        .with_state(repo)
        .layer(from_fn(auth));

    public.merge(protected)
}
