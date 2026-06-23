use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, Iden, PaginatorTrait, QueryFilter};
use uuid::Uuid;
use crate::entities::prelude::Todos;
use crate::models::{ApiResponse, CreateTodoReq, CreateTodoResp, FindTodoResp, FlagDoneTodoResp, GetTodoResp, PaginationQuery, PaginationResp, UpdateTodoResp};
use sea_orm::EntityTrait;
use crate::entities::todos;
use sea_orm::ActiveValue::Set;
use sea_orm::sqlx::types::chrono;
use validator::Validate;
use crate::entities::todos::Column;
use crate::middleware::AuthGuard;

pub async fn get_todos(
    State(store): State<DatabaseConnection>,
    auth: AuthGuard,
    Query(params): Query<PaginationQuery>
) -> (StatusCode, Json<ApiResponse<PaginationResp<Vec<GetTodoResp>>>>) {

    // get time in
    let t_in = Utc::now();

    // get paginator
    let page = params.page.unwrap_or(1);
    let size = params.size.unwrap_or(10);

    let paginator = Todos::find()
        .filter(Column::UserId.eq(auth.user_id))
        .paginate(&store, size);
    let total = paginator.num_items().await.unwrap();
    let data_model = paginator.fetch_page(page - 1).await.unwrap();

    // convert to list
    let data_list = data_model.into_iter()
        .map(|data| GetTodoResp {
            id: data.id,
            title: data.title,
            description: data.description,
            is_completed: data.is_completed,
            created_at: data.created_at,
        }).collect();

    let resp = PaginationResp {
        list: data_list,
        total,
        page,
        limit: size,
        total_pages: (total as f64 / size as f64).ceil() as u64,
    };

    (StatusCode::OK, Json(ApiResponse::success(t_in, StatusCode::OK.as_u16(), resp)))
}

pub async fn create_todo(
    State(store): State<DatabaseConnection>,
    auth: AuthGuard,
    Json(body): Json<CreateTodoReq>,
) -> (StatusCode, Json<ApiResponse<CreateTodoResp>>) {

    // get time in
    let t_in = Utc::now();

    // validate
    if let Err(errors) = body.validate() {

        let err_msg = errors
            .field_errors()
            .into_iter()
            .map(|(_, e)| e[0].message.clone().unwrap_or_default())
            .collect::<Vec<_>>()
            .join(", ");
        
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::error(t_in, StatusCode::BAD_REQUEST.as_u16(), err_msg)))
    }

    // generate uuid
    let uuid = Uuid::new_v4();

    // generate db
    let new_todo = todos::ActiveModel{
        id: Set(uuid),
        user_id: Set(auth.user_id),
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

            // check user_id valid
            if data.user_id != auth.user_id {
                return (StatusCode::NOT_FOUND, Json(ApiResponse::error(t_in, StatusCode::NOT_FOUND.as_u16(), "data not found".to_string())))
            }

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

            // check user id
            if data.user_id == auth.user_id {
                return (StatusCode::NOT_FOUND, Json(ApiResponse::error(t_in, StatusCode::NOT_FOUND.as_u16(), "data not found".to_string())))
            }

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

    if let Err(errors) = body.validate() {
        let err_msg = errors
            .field_errors()
            .into_iter()
            .map(|(_, e)| e[0].message.clone().unwrap_or_default())
            .collect::<Vec<_>>()
            .join(", ");

        return (StatusCode::BAD_REQUEST, Json(ApiResponse::error(t_in, StatusCode::BAD_REQUEST.as_u16(), err_msg)))
    }
    
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

            // check user_id
            if data.user_id == auth.user_id {
                return  (StatusCode::NOT_FOUND, Json(ApiResponse::error(t_in, StatusCode::NOT_FOUND.as_u16(), "data not found".to_string())))
            }

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