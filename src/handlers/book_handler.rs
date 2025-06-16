use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use crate::{
    domain::book::Book,
    app::book_repository::BookRepository,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub published_year: Option<i32>,
}

pub async fn get_books<R: BookRepository>(
    State(repo): State<Arc<R>>,
) -> Result<Json<Vec<Book>>, (StatusCode, String)> {
    repo.get_all()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn get_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<String>,
) -> Result<Json<Book>, (StatusCode, String)> {
    match repo.get_by_id(&id).await {
        Ok(Some(book)) => Ok(Json(book)),
        Ok(None) => Err((StatusCode::NOT_FOUND, format!("Book {} not found", id))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn post_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Json(payload): Json<CreateBook>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)> {
    // Construimos la entidad Book en el dominio
    let book = Book::new(payload.title, payload.author, payload.published_year);

    match repo.create(book.clone()).await {
        Ok(saved) => Ok((StatusCode::CREATED, Json(saved))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
