use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("Unauthorized")]
    Auth,

    #[error(transparent)]
    Db(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::NotFound(_)   => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Auth          => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Db(_)         => {
                tracing::error!("DB error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".into())
            }
        };
        let body = Json(ErrorBody { error: msg });
        (status, body).into_response()
    }
}
