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
    error::AppError,
};

#[derive(Deserialize, Validate)]
pub struct CreateBook {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,

    #[validate(length(min = 1, message = "Author cannot be empty"))]
    pub author: String,

    #[validate(range(min = 0, message = "Published year must be positive"))]
    pub published_year: Option<i32>,
}

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
) -> Result<Json<Vec<Book>>, AppError> {
    let books = repo.get_all().await?;
    Ok(Json(books))
}

pub async fn get_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<String>,
) -> Result<Json<Book>, AppError> {
    let book = repo
        .get_by_id(&id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Book {} not found", id)))?;
    Ok(Json(book))
}

pub async fn post_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Json(payload): Json<CreateBook>,
) -> Result<(StatusCode, Json<Book>), AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let book = Book::new(payload.title, payload.author, payload.published_year);
    let saved = repo.create(book).await?;
    Ok((StatusCode::CREATED, Json(saved)))
}

pub async fn put_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBook>,
) -> Result<Json<Book>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    let mut book = repo
        .get_by_id(&id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Book {} not found", id)))?;
    if let Some(title) = payload.title { book.title = title; }
    if let Some(author) = payload.author { book.author = author; }
    if payload.published_year.is_some() {
        book.published_year = payload.published_year;
    }
    let updated = repo.update(book).await?;
    Ok(Json(updated))
}

pub async fn delete_book<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    repo.delete(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn search_books<R: BookRepository>(
    State(repo): State<Arc<R>>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Book>>, AppError> {
    let title = params.title.as_deref();
    let author = params.author.as_deref();
    let books = repo.search(title, author).await?;
    Ok(Json(books))
}
