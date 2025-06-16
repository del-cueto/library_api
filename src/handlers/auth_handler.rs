use axum::Json;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::Utc;
use crate::{config::jwt_secret, error::AppError};

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(Json(payload): Json<Login>) -> Result<Json<String>, AppError> {
    if payload.username != "admin" || payload.password != "password" {
        return Err(AppError::Auth);
    }

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims { sub: payload.username.clone(), exp: expiration };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_ref()),
    )
        .map_err(|e| AppError::Db(e.into()))?;

    Ok(Json(token))
}
