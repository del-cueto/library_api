use axum::{Json, extract::State};
use crate::{domain::book::Book, app::book_repository::BookRepository};
use std::sync::Arc;

pub async fn get_books<R: BookRepository>(
    State(repo): State<Arc<R>>,
) -> Result<Json<Vec<Book>>, (axum::http::StatusCode, String)> {
    match repo.get_all().await {
        Ok(books) => Ok(Json(books)),
        Err(_) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch books".into())),
    }
}
