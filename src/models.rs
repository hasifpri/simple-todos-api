use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize)]
pub struct DataTodo {
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

#[derive(Serialize,Debug,Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

#[derive(Deserialize)]
pub struct RegisterUserReq {
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterUserResp {
    pub id: Uuid,
    pub name: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct LoginUserReq {
    pub username: String,
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