use crate::common::AppState;
use crate::models::{Host, NewHost};
use crate::schema;
use diesel::QueryDsl;
use diesel::prelude::*;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::Utc;
use diesel_async::RunQueryDsl;

#[axum::debug_handler]
pub async fn list(State(state): State<AppState>) -> (StatusCode, Json<Vec<Host>>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let hosts: Vec<Host> = schema::host::table.load(&mut read_conn).await.unwrap();

    println!("Hosts {:?}", hosts);

    (StatusCode::OK, Json(hosts))
}

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    Path(host_id): Path<i32>,
) -> (StatusCode, Json<Host>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let host: Host = schema::host::table
        .filter(schema::host::id.eq(host_id))
        .first(&mut read_conn)
        .await
        .unwrap();

    println!("Hosts {:?}", host);

    (StatusCode::OK, Json(host))
}

#[axum::debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Json(new_host): Json<NewHost>,
) -> (StatusCode, Json<Host>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let host: Host = diesel::insert_into(schema::host::table)
        .values(&new_host)
        .get_result::<Host>(&mut read_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(host))
}

#[axum::debug_handler]
pub async fn update(
    State(state): State<AppState>,
    Path(host_id): Path<i32>,
    Json(host_data): Json<NewHost>,
) -> (StatusCode, Json<Host>) {
    use crate::schema::host::dsl::*;
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let updated_host: Host = diesel::update(host.filter(id.eq(host_id)))
        .set((
            group_id.eq(host_data.group_id),
            name.eq(host_data.name),
            hostname.eq(host_data.hostname),
            port.eq(host_data.port),
            username.eq(host_data.username),
            password.eq(host_data.password),
            updated_at.eq(Utc::now()),
        ))
        .get_result(&mut read_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(updated_host))
}

#[axum::debug_handler]
pub async fn delete(State(state): State<AppState>, Path(host_id): Path<i32>) -> StatusCode {
    use crate::schema::host::dsl::*;
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    diesel::delete(host.filter(id.eq(host_id)))
        .execute(&mut read_conn)
        .await
        .unwrap();

    StatusCode::OK
}
