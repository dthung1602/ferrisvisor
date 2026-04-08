use crate::common::AppState;
use crate::models::{NewPermission, Permission, UpdatePermission};
use crate::schema;
use diesel::QueryDsl;
use diesel::prelude::*;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use diesel_async::RunQueryDsl;

#[axum::debug_handler]
pub async fn list(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> (StatusCode, Json<Vec<Permission>>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let match_user = schema::permission::user_id.eq(user_id);
    let permissions: Vec<Permission> = schema::permission::table
        .filter(match_user)
        .load(&mut read_conn)
        .await
        .unwrap();

    println!("Permissions {:?}", permissions);

    (StatusCode::OK, Json(permissions))
}

#[axum::debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Json(new_permission): Json<NewPermission>,
) -> (StatusCode, Json<Permission>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let permission: Permission = diesel::insert_into(schema::permission::table)
        .values(new_permission)
        .get_result::<Permission>(&mut read_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(permission))
}

#[axum::debug_handler]
pub async fn update(
    State(state): State<AppState>,
    Path((user_id, permission_id)): Path<(i32, i32)>,
    Json(permission_data): Json<UpdatePermission>,
) -> (StatusCode, Json<Permission>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let match_user_perm = schema::permission::id
        .eq(permission_id)
        .and(schema::permission::user_id.eq(user_id));

    let updated_permission: Permission =
        diesel::update(schema::permission::table.filter(match_user_perm))
            .set((
                schema::permission::host_id.eq(permission_data.host_id),
                schema::permission::service_name.eq(permission_data.service_name),
                schema::permission::can_view.eq(permission_data.can_view),
                schema::permission::can_act.eq(permission_data.can_act),
            ))
            .get_result(&mut read_conn)
            .await
            .unwrap();

    (StatusCode::OK, Json(updated_permission))
}

#[axum::debug_handler]
pub async fn delete(
    State(state): State<AppState>,
    Path((user_id, permission_id)): Path<(i32, i32)>,
) -> StatusCode {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let match_user_perm = schema::permission::id
        .eq(permission_id)
        .and(schema::permission::user_id.eq(user_id));

    diesel::delete(schema::permission::table.filter(match_user_perm))
        .execute(&mut read_conn)
        .await
        .unwrap();

    StatusCode::OK
}
