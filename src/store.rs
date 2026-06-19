use std::sync::{Arc, Mutex};
use uuid::Uuid;
use std::collections::HashMap;
use crate::models::Todo;

pub type TodoStore = Arc<Mutex<HashMap<Uuid, Todo>>>;

pub fn new_store() -> TodoStore {
    Arc::new(
        Mutex::new(
            HashMap::new()
        )
    )
}