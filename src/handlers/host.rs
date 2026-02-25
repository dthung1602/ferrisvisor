use crate::common::AppState;
use crate::models::{Host, NewHost};
use crate::schema;

use axum::Json;
use axum::extract::State;
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
