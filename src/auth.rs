use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use crate::entities::prelude::Users;
use crate::entities::users;
use crate::entities::users::Column;
use crate::models::{Claims, LoginUserReq, LoginUserResp, RegisterUserReq, RegisterUserResp};


pub async fn register_user(
    State(store): State<DatabaseConnection>,
    Json(body): Json<RegisterUserReq>
) -> (StatusCode, Json<Option<RegisterUserResp>>) {

    // generate uuid
    let uuid = Uuid::new_v4();

    // hash password
    let hashed_password = hash(&body.password, DEFAULT_COST).unwrap();

    // generate db
    let users_model = users::ActiveModel {
        id: Set(uuid),
        name: Set(body.name),
        username: Set(body.username),
        password: Set(hashed_password),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
    };

    // insert
    let result = users_model.insert(&store).await;

    match result {
        Ok(data) => {

            let users_resp = RegisterUserResp {
                id: data.id,
                name: data.name,
                username: data.username,
            };

            (StatusCode::CREATED, Json(Some(users_resp)))
        }
        Err(_) => {

            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }

}

pub async fn login_user(
    State(store): State<DatabaseConnection>,
    Json(body): Json<LoginUserReq>,
) -> (StatusCode, Json<Option<LoginUserResp>>) {

    // get find
    let user_data = Users::find()
        .filter(Column::Username.eq(&body.username))
        .one(&store).await;

    match user_data {
        Ok(Some(data)) => {

            let is_pass_valid = verify(&body.password, &data.password).unwrap();

            if is_pass_valid {

                // generate claims
                let claims = Claims {
                    sub: data.id.to_string(),
                    name: data.name.to_string(),
                    username: data.username.to_string(),
                    exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
                };

                let jwt_key = std::env::var("JWT_KEY").expect("JWT_KEY mus be filled");

                // generate token
                let result = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(jwt_key.as_bytes()),
                ).unwrap();

                // generate resp
                let resp = LoginUserResp{
                    token: result,
                };

                return (StatusCode::OK, Json(Some(resp)))
            }

            (StatusCode::UNAUTHORIZED, Json(None))

        },
        Ok(None) => {
            (StatusCode::UNAUTHORIZED, Json(None))
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }

}