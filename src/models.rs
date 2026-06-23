use chrono::{DateTime, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize,Validate)]
pub struct CreateTodoReq {
    #[validate(length(min=1, message="title must be filled and min 5 char"))]
    pub title: String,

    #[validate(length(min=5, message="description must be filled and min 5 char"))]
    pub description: String,

    pub is_completed: bool,
}

#[derive(Serialize)]
pub struct CreateTodoResp {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Serialize)]
pub struct FindTodoResp {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Serialize)]
pub struct UpdateTodoResp {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct FlagDoneTodoResp {
    pub id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct RegisterUserReq {
    #[validate(length(min=5, message="name must be filled and min 5 char"))]
    pub name: String,

    #[validate(length(min=5, message="username must be filled and min 5 char"))]
    pub username: String,

    #[validate(length(min=8, message="password mus be filled and min 8 char"))]
    pub password: String,

    #[validate(email(message="email must be filled"))]
    pub email: String,
}

#[derive(Serialize)]
pub struct RegisterUserResp {
    pub id: Uuid,
    pub name: String,
    pub username: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUserReq {
    #[validate(length(min=5, message="username must be filled and min 5 char"))]
    pub username: String,

    #[validate(length(min=8, message="password must be filled and min 8 char"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginUserResp {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub username: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub size: Option<u64>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub latency: String,
    pub error: Option<String>,
    pub tin: DateTime<Utc>,
    pub tout: DateTime<Utc>,
    pub success: bool,
    pub status: u16,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct PaginationResp<T> {
    pub list: T,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

#[derive(Serialize)]
pub struct GetTodoResp {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Serialize)]
pub struct AuthMeResp {
    pub user_id: Uuid,
    pub name: String,
    pub username: String,
}