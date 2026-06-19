use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use crate::models::{DataTodo, Todo};
use crate::store::TodoStore;

pub async fn get_todos(
    State(store): State<TodoStore>
) -> Json<Vec<Todo>> {
    let store = store.lock().unwrap();

    let todos: Vec<Todo> = store.values().cloned().collect();

    Json(todos)
}

pub async fn create_todo(
    State(store): State<TodoStore>,
    Json(body): Json<DataTodo>,
) -> (StatusCode, Json<Todo>) {

    // generate uuid
    let uuid = Uuid::new_v4();

    let todo = Todo{
        id: uuid,
        title: body.title,
        description: body.description,
        is_completed: false,
    };

    let mut db = store.lock().unwrap();
    db.insert(uuid, todo.clone());

    (StatusCode::CREATED, Json(todo))
    
}

pub async fn delete_todo(
    State(store): State<TodoStore>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let mut db = store.lock().unwrap();

    match db.remove(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND
    }
}

pub async fn find_todo(
    State(store): State<TodoStore>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<Option<Todo>>) {
    let db = store.lock().unwrap();

    match db.get(&id) {
        Some(data) => (StatusCode::OK, Json(Some(data.clone()))),
        None => (StatusCode::NOT_FOUND, Json(None))
    }
}

pub async fn update_todo(
    State(store): State<TodoStore>,
    Path(id): Path<Uuid>,
    Json(body): Json<DataTodo>,
) -> (StatusCode, Json<Option<Todo>>) {
    let mut db = store.lock().unwrap();

    match db.get_mut(&id) {
        Some(data) => {

            data.title = body.title;
            data.description = body.description;
            data.is_completed = body.is_completed;

            (StatusCode::OK, Json(Some(data.clone())))
        }
        None => (StatusCode::NOT_FOUND, Json(None))
    }

}

pub async fn flag_done_todo(
    State(store): State<TodoStore>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<Option<Todo>>) {
    
    // get mutex db
    let mut db = store.lock().unwrap();

    match db.get_mut(&id) {
        Some(data) => {
            data.is_completed = true;

            (StatusCode::OK, Json(Some(data.clone())))
        },
        None => {
            (StatusCode::NOT_FOUND, Json(None))
        }
    }
}