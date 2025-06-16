use axum::{Json, extract::{Path, State}};
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

pub async fn get_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<String>,
) -> Result<Json<Book>, (axum::http::StatusCode, String)> {
    match repo.get_by_id(&id).await {
        Ok(Some(book)) => Ok(Json(book)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, format!("Book {} not found", id))),
        Err(_) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error fetching book".into())),
    }
}
