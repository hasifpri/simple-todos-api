use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use axum::Json;
use uuid::Uuid;
use crate::models::{ApiResponse, Claims};

pub struct AuthGuard {
    pub user_id: Uuid,
    pub name: String,
    pub username: String,
}

impl<S> FromRequestParts<S> for AuthGuard
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApiResponse<()>>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {

        let t_in = Utc::now();

        // get header
        let auth_header = parts.headers.get("Authorization");

        match auth_header {
            Some(data) => {

                // get token
                let token = data.to_str().unwrap().replace("Bearer ", "");

                // generate jwt
                let jwt_key = std::env::var("JWT_KEY").unwrap();

                // decode
                let decoded = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(jwt_key.as_bytes()),
                    &Validation::default(),
                ).map_err(|_| (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::error(t_in, StatusCode::UNAUTHORIZED.as_u16(), "invalid or expired token".to_string()))
                ))?;

                Ok(AuthGuard {
                    user_id: Uuid::parse_str(&decoded.claims.sub).unwrap(),
                    name: decoded.claims.name,
                    username: decoded.claims.username,
                })

            },
            None => return Err( (StatusCode::UNAUTHORIZED, Json(ApiResponse::error(t_in, StatusCode::UNAUTHORIZED.as_u16(), "token is required".to_string()))))
        }
    }
}