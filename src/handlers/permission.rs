use crate::common::AppState;
use crate::models::{DisplayPermission, NewPermission, Permission, UpdatePermission};
use crate::schema;
use diesel::prelude::*;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use diesel_async::RunQueryDsl;

#[axum::debug_handler]
pub async fn list(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> (StatusCode, Json<Vec<DisplayPermission>>) {
    let mut db_conn = state.db_conn.lock().await;

    let permissions_with_extra = schema::permission::table
        .inner_join(schema::group::table)
        .left_join(schema::host::table)
        .filter(schema::permission::user_id.eq(user_id))
        .select((
            Permission::as_select(),
            schema::host::name.nullable(),
            schema::group::name,
        ))
        .order_by((
            schema::permission::group_id.asc(),
            schema::permission::host_id.asc(),
            schema::permission::id.asc(),
        ))
        .load::<(Permission, Option<String>, String)>(&mut db_conn)
        .await
        .unwrap();

    println!("Permissions {:?}", permissions_with_extra);

    let permissions_with_extra: Vec<DisplayPermission> = permissions_with_extra
        .into_iter()
        .map(|(perm, host_name, group_name)| {
            DisplayPermission::from_perm(perm, host_name, group_name)
        })
        .collect();

    (StatusCode::OK, Json(permissions_with_extra))
}

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    Path((user_id, permission_id)): Path<(i32, i32)>,
) -> (StatusCode, Json<DisplayPermission>) {
    let mut db_conn = state.db_conn.lock().await;

    let (permission, host_name, group_name) = schema::permission::table
        .inner_join(schema::group::table)
        .left_join(schema::host::table)
        .filter(schema::permission::user_id.eq(user_id))
        .select((
            Permission::as_select(),
            schema::host::name.nullable(),
            schema::group::name,
        ))
        .order_by((
            schema::group::name.asc(),
            schema::host::name.asc(),
            schema::permission::id.asc(),
        ))
        .first::<(Permission, Option<String>, String)>(&mut db_conn)
        .await
        .unwrap();

    let permission = DisplayPermission::from_perm(permission, host_name, group_name);

    (StatusCode::OK, Json(permission))
}

#[axum::debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(new_permission): Json<UpdatePermission>,
) -> (StatusCode, Json<Option<Permission>>) {
    let mut db_conn = state.db_conn.lock().await;

    if let Some(host_id) = new_permission.host_id {
        use schema::host::*;
        let count: i64 = table
            .filter(id.eq(host_id).and(group_id.eq(new_permission.group_id)))
            .count()
            .get_result(&mut db_conn)
            .await
            .unwrap();
        if count == 0 {
            return (StatusCode::BAD_REQUEST, Json(None));
        }
    }

    let new_permission = NewPermission::new(
        user_id,
        new_permission.group_id,
        new_permission.host_id,
        &new_permission.service_name,
        new_permission.can_view,
        new_permission.can_act,
    );

    println!("\nNew permission --> {:?}\n", new_permission);

    let permission: Permission = diesel::insert_into(schema::permission::table)
        .values(new_permission)
        .get_result::<Permission>(&mut db_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(Some(permission)))
}

#[axum::debug_handler]
pub async fn update(
    State(state): State<AppState>,
    Path((user_id, permission_id)): Path<(i32, i32)>,
    Json(permission_data): Json<UpdatePermission>,
) -> (StatusCode, Json<Option<Permission>>) {
    let mut db_conn = state.db_conn.lock().await;

    if let Some(host_id) = permission_data.host_id {
        use schema::host::*;
        let count: i64 = table
            .filter(id.eq(host_id).and(group_id.eq(permission_data.group_id)))
            .count()
            .get_result(&mut db_conn)
            .await
            .unwrap();
        if count == 0 {
            return (StatusCode::BAD_REQUEST, Json(None));
        }
    }

    let match_user_perm = schema::permission::id
        .eq(permission_id)
        .and(schema::permission::user_id.eq(user_id));

    let updated_permission: Permission =
        diesel::update(schema::permission::table.filter(match_user_perm))
            .set((
                schema::permission::host_id.eq(permission_data.host_id),
                schema::permission::service_name.eq(permission_data.service_name),
                // can_act implies can_view
                schema::permission::can_view.eq(permission_data.can_view | permission_data.can_act),
                schema::permission::can_act.eq(permission_data.can_act),
            ))
            .get_result(&mut db_conn)
            .await
            .unwrap();

    (StatusCode::OK, Json(Some(updated_permission)))
}

#[axum::debug_handler]
pub async fn delete(
    State(state): State<AppState>,
    Path((user_id, permission_id)): Path<(i32, i32)>,
) -> StatusCode {
    let mut db_conn = state.db_conn.lock().await;

    let match_user_perm = schema::permission::id
        .eq(permission_id)
        .and(schema::permission::user_id.eq(user_id));

    diesel::delete(schema::permission::table.filter(match_user_perm))
        .execute(&mut db_conn)
        .await
        .unwrap();

    StatusCode::OK
}
