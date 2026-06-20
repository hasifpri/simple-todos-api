use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::Claims;

pub struct AuthGuard {
    pub user_id: String,
    pub name: String,
    pub username: String,
}

impl<S> FromRequestParts<S> for AuthGuard
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {

        // get header
        let auth_header = parts.headers.get("Authorization").ok_or(StatusCode::UNAUTHORIZED)?;

        // get token
        let token = auth_header.to_str().unwrap().replace("Bearer ", "");

        // generate jwt
        let jwt_key = std::env::var("JWT_KEY").unwrap();

        // decode
        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_key.as_bytes()),
            &Validation::default(),
        ).map_err(|_| { StatusCode::UNAUTHORIZED })?;

        Ok(AuthGuard {
            user_id: decoded.claims.sub,
            name: decoded.claims.name,
            username: decoded.claims.username,
        })
    }
}