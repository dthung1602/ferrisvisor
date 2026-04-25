use crate::common::AppState;
use crate::models::{Group, NewGroup};
use crate::schema;
use diesel::QueryDsl;
use diesel::prelude::*;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::Utc;
use diesel_async::RunQueryDsl;

#[axum::debug_handler]
pub async fn list(State(state): State<AppState>) -> (StatusCode, Json<Vec<Group>>) {
    let mut db_conn = state.db_pool.get().await.unwrap();

    let groups: Vec<Group> = schema::group::table.load(&mut db_conn).await.unwrap();

    println!("Groups {:?}", groups);

    (StatusCode::OK, Json(groups))
}

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    Path(group_id): Path<i32>,
) -> (StatusCode, Json<Group>) {
    let mut db_conn = state.db_pool.get().await.unwrap();

    let group: Group = schema::group::table
        .filter(schema::group::id.eq(group_id))
        .first(&mut db_conn)
        .await
        .unwrap();

    println!("Group {:?}", group);

    (StatusCode::OK, Json(group))
}

#[axum::debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Json(new_group): Json<NewGroup>,
) -> (StatusCode, Json<Group>) {
    let mut db_conn = state.db_pool.get().await.unwrap();

    let group: Group = diesel::insert_into(schema::group::table)
        .values(&new_group)
        .get_result::<Group>(&mut db_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(group))
}

#[axum::debug_handler]
pub async fn update(
    State(state): State<AppState>,
    Path(group_id): Path<i32>,
    Json(group_data): Json<NewGroup>,
) -> (StatusCode, Json<Group>) {
    use crate::schema::group::dsl::*;
    let mut db_conn = state.db_pool.get().await.unwrap();

    let updated_host: Group = diesel::update(group.filter(id.eq(group_id)))
        .set((
            name.eq(group_data.name),
            description.eq(group_data.description),
            color.eq(group_data.color),
            updated_at.eq(Utc::now()),
        ))
        .get_result(&mut db_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(updated_host))
}

#[axum::debug_handler]
pub async fn delete(State(state): State<AppState>, Path(group_id): Path<i32>) -> StatusCode {
    use crate::schema::group::dsl::*;
    let mut db_conn = state.db_pool.get().await.unwrap();

    diesel::delete(group.filter(id.eq(group_id)))
        .execute(&mut db_conn)
        .await
        .unwrap();

    StatusCode::OK
}
