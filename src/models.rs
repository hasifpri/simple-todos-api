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