use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

use crate::{
    domain::book::Book,
    app::book_repository::BookRepository,
};

/// POST /books con validaciones
#[derive(Deserialize, Validate)]
pub struct CreateBook {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "Author cannot be empty"))]
    pub author: String,

    #[validate(range(min = 0, message = "Published year must be positive"))]
    pub published_year: Option<i32>,
}

///  PUT /books/:id con validaciones
#[derive(Deserialize, Validate)]
pub struct UpdateBook {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: Option<String>,

    #[validate(length(min = 1, message = "Author cannot be empty"))]
    pub author: Option<String>,

    #[validate(range(min = 0, message = "Published year must be positive"))]
    pub published_year: Option<i32>,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub title: Option<String>,
    pub author: Option<String>,
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
        Ok(None)       => Err((StatusCode::NOT_FOUND, format!("Book {} not found", id))),
        Err(e)         => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn post_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Json(payload): Json<CreateBook>,
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    // create and persist
    let book = Book::new(payload.title, payload.author, payload.published_year);
    repo.create(book.clone())
        .await
        .map(|saved| (StatusCode::CREATED, Json(saved)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn put_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let mut book = match repo.get_by_id(&id).await {
        Ok(Some(b)) => b,
        Ok(None)    => return Err((StatusCode::NOT_FOUND, format!("Book {} not found", id))),
        Err(e)      => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    if let Some(title) = payload.title { book.title = title; }
    if let Some(author) = payload.author { book.author = author; }
    if payload.published_year.is_some() { book.published_year = payload.published_year; }

    // Persist
    repo.update(book.clone())
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn delete_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    repo.delete(&id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// GET /books/search?title=…&author=…
pub async fn search_books<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Book>>, (StatusCode, String)> {
    let title = params.title.as_deref();
    let author = params.author.as_deref();

    repo.search(title, author)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
