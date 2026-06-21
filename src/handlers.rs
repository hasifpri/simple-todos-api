use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Iden};
use uuid::Uuid;
use crate::entities::prelude::Todos;
use crate::models::{ApiResponse, CreateTodoReq, CreateTodoResp, FindTodoResp, FlagDoneTodoResp, UpdateTodoResp};
use sea_orm::EntityTrait;
use crate::entities::todos;
use sea_orm::ActiveValue::Set;
use sea_orm::sqlx::types::chrono;
use crate::middleware::AuthGuard;

pub async fn get_todos(
    State(store): State<DatabaseConnection>,
    auth: AuthGuard,
) -> (StatusCode, Json<ApiResponse<Vec<todos::Model>>>) {

    // get time in
    let t_in = Utc::now();

    let result = Todos::find()
        .all(&store)
        .await;

    match result {
        Ok(data) => {
            (StatusCode::OK, Json(ApiResponse::success(t_in, StatusCode::OK.as_u16(), data)))
        },
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string())))
        }
    }
}

pub async fn create_todo(
    State(store): State<DatabaseConnection>,
    auth: AuthGuard,
    Json(body): Json<CreateTodoReq>,
) -> (StatusCode, Json<ApiResponse<CreateTodoResp>>) {

    // get time in
    let t_in = Utc::now();

    // generate uuid
    let uuid = Uuid::new_v4();

    // generate db
    let new_todo = todos::ActiveModel{
        id: Set(uuid),
        title: Set(body.title),
        description: Set(body.description),
        is_completed: Set(false),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
    };

    // insert
    let result = new_todo.insert(&store).await;

    match result {
        Ok(data) => {

            // convert to resp
            let resp = CreateTodoResp {
                id: data.id,
                title: data.title,
                description: data.description,
                is_completed: data.is_completed,
                created_at: data.created_at,
                updated_at: data.updated_at,
            };

            (StatusCode::CREATED, Json(ApiResponse::success(t_in, StatusCode::OK.as_u16(), resp)))
        },
        Err(err)=> {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string())))
        }
    }

}

pub async fn delete_todo(
    State(store): State<DatabaseConnection>,
    auth: AuthGuard,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<ApiResponse<()>>) {

    let t_in = Utc::now();

    let result = Todos::find_by_id(id).one(&store).await;

    match result {
        Ok(Some(data)) => {
            let active: todos::ActiveModel = data.into();

            match active.delete(&store).await {
                Ok(_) => (StatusCode::NO_CONTENT, Json(ApiResponse::success(t_in, StatusCode::NO_CONTENT.as_u16(), ()))),
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string()))),
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(ApiResponse::error(t_in, StatusCode::NOT_FOUND.as_u16(), "data not found".to_string()))),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string()))),
    }
}

pub async fn find_todo(
    State(store): State<DatabaseConnection>,
    auth: AuthGuard,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<ApiResponse<FindTodoResp>>) {

    let t_in = Utc::now();

    // get data
    let data_selected = Todos::find_by_id(id).one(&store).await;

    match data_selected {
        Ok(Some(data)) => {
            let resp = FindTodoResp {
                id: data.id,
                title: data.title,
                description: data.description,
                is_completed: data.is_completed,
                created_at: data.created_at,
                updated_at: data.updated_at
            };

            (StatusCode::OK, Json(ApiResponse::success(t_in, StatusCode::OK.as_u16(), resp)))
        },
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(ApiResponse::error(t_in, StatusCode::NOT_FOUND.as_u16(), "data not found".to_string())))
        },
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string())))
        },
    }
}

pub async fn update_todo(
    State(store): State<DatabaseConnection>,
    auth: AuthGuard,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateTodoReq>,
) -> (StatusCode, Json<ApiResponse<UpdateTodoResp>>) {

    let t_in = Utc::now();


    // find data
    let data_selected = Todos::find_by_id(id).one(&store).await;

    match data_selected {
        Ok(Some( data)) => {

            // set data active
            let mut data_active: todos::ActiveModel = data.into();

            // update data
            data_active.title = Set(body.title);
            data_active.description = Set(body.description);
            data_active.is_completed = Set(body.is_completed);
            data_active.updated_at = Set(Utc::now().into());

            // exec update
            let result = data_active.update(&store).await;

            match result {
                Ok(data) => {
                    let resp = UpdateTodoResp {
                        id: data.id
                    };

                    (StatusCode::OK, Json(ApiResponse::success(t_in, StatusCode::OK.as_u16(), resp)))
                },
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string())))
            }

        },
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(ApiResponse::error(t_in, StatusCode::NOT_FOUND.as_u16(), "data not found".to_string())))
        },
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string())))
        }
    }

}

pub async fn flag_done_todo(
    State(store): State<DatabaseConnection>,
    auth: AuthGuard,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<ApiResponse<FlagDoneTodoResp>>) {
    let t_in = Utc::now();

    // get single data
    let single_data = Todos::find_by_id(id).one(&store).await;

    match single_data {
        Ok(Some(data)) => {
            let mut data_active: todos::ActiveModel = data.into();

            // flag done
            data_active.is_completed = Set(true);
            data_active.updated_at = Set(Utc::now().into());

            // exec update
            let result_exec = data_active.update(&store).await;

            match result_exec {
                Ok(data) => {
                    let resp = FlagDoneTodoResp {
                        id: data.id
                    };

                    (StatusCode::OK, Json(ApiResponse::success(t_in, StatusCode::OK.as_u16(), resp)))
                },
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string())))
            }
        },
        Ok(None) => (StatusCode::NOT_FOUND, Json(ApiResponse::error(t_in, StatusCode::NOT_FOUND.as_u16(), "data not found".to_string()))),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::error(t_in, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), err.to_string()))),
    }
}