use axum::{
    body::Body,
    http::{Request, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::Value;
use crate::{config::jwt_secret, error::AppError};

pub async fn auth(req: Request<Body>, next: Next) -> Response {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(header) = auth_header {
        if let Some(token) = header.strip_prefix("Bearer ") {
            if decode::<Value>(
                token,
                &DecodingKey::from_secret(jwt_secret().as_ref()),
                &Validation::default(),
            )
                .is_ok()
            {
                return next.run(req).await;
            }
        }
    }

    AppError::Auth.into_response()
}
