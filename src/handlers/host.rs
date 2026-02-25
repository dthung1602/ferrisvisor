use crate::common::AppState;
use crate::models::{Host, NewHost, UpdateHost};
use crate::schema;
use diesel::QueryDsl;
use diesel::prelude::*;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
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
    Json(host_data): Json<UpdateHost>,
) -> (StatusCode, Json<Host>) {
    use crate::schema::host::dsl::*;
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let updated_host: Host = diesel::update(host.filter(id.eq(host_data.id)))
        .set((
            name.eq(host_data.name),
            port.eq(host_data.port),
            username.eq(host_data.username),
            password.eq(host_data.password),
        ))
        .get_result(&mut read_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(updated_host))
}
