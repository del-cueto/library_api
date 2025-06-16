use axum::{
    body::Body,
    http::{Request, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::Value;
use crate::config::jwt_secret;

/// JWT guard: returns a full `Response<Body>` in both branches.
pub async fn auth(req: Request<Body>, next: Next) -> Response {
    //Extract the header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    //If we have “Bearer <token>” and it decodes OK, forward the request
    if let Some(header) = auth_header {
        if let Some(token) = header.strip_prefix("Bearer ") {
            if decode::<Value>(
                token,
                &DecodingKey::from_secret(jwt_secret().as_ref()),
                &Validation::default(),
            )
                .is_ok()
            {
                // next.run(...) returns a Response<Body>
                return next.run(req).await;
            }
        }
    }

    //Otherwise build a 401 Response<Body> from the tuple
    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}
