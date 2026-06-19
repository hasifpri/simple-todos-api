use axum::Router;
use axum::routing::{delete, get, patch, post, put};

mod models;
mod store;
mod handlers;

#[tokio::main]
async fn main() {

    // create store
    let store = store::new_store();

    // generate route
    let app = Router::new()
        .route("/todos", get(handlers::get_todos))
        .route("/todos", post(handlers::create_todo))
        .route("/todos/{id}", get(handlers::find_todo))
        .route("/todos/{id}", put(handlers::update_todo))
        .route("/todos/{id}", delete(handlers::delete_todo))
        .route("/todos/{id}/flag-done", patch(handlers::flag_done_todo))
        .with_state(store);

    // run server
    let port = "3000".to_string();
    let listener = tokio::net::TcpListener::bind( format!("0.0.0.0:{}",port)).await.unwrap();

    println!("listen on port: {}",port);

    // run axum
    axum::serve(listener, app).await.unwrap();
}


