use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::config::jwt_secret;

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

/// POST /login
pub async fn login(Json(payload): Json<Login>) -> Result<Json<String>, (StatusCode, String)> {
    if payload.username != "admin" || payload.password != "password" {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".into()));
    }

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: payload.username.clone(),
        exp: expiration,
    };
    
    //sign the token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_ref()),
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(token))
}
