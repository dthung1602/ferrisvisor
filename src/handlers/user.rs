use crate::common::AppState;
use crate::models::{HasPassword, NewUser, ResponseUser, UpdateUser, User};
use crate::schema;
use diesel::QueryDsl;
use diesel::prelude::*;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::Utc;
use diesel_async::RunQueryDsl;

#[axum::debug_handler]
pub async fn list(State(state): State<AppState>) -> (StatusCode, Json<Vec<ResponseUser>>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let users: Vec<User> = schema::user::table.load(&mut read_conn).await.unwrap();

    println!("Users {:?}", users);

    let users = users.iter().map(ResponseUser::from_user).collect();

    (StatusCode::OK, Json(users))
}

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> (StatusCode, Json<ResponseUser>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let user: User = schema::user::table
        .filter(schema::user::id.eq(user_id))
        .first(&mut read_conn)
        .await
        .unwrap();

    println!("Users {:?}", user);

    (StatusCode::OK, Json(user.into()))
}

#[axum::debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Json(mut new_user): Json<NewUser>,
) -> (StatusCode, Json<ResponseUser>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    // hash password
    new_user.set_password(&new_user.password.clone());

    let user: User = diesel::insert_into(schema::user::table)
        .values(new_user)
        .get_result::<User>(&mut read_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(user.into()))
}

#[axum::debug_handler]
pub async fn update(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(user_data): Json<UpdateUser>,
) -> (StatusCode, Json<ResponseUser>) {
    use crate::schema::user::dsl::*;
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let updated_user: User = diesel::update(user.filter(id.eq(user_id)))
        .set((
            email.eq(user_data.email),
            is_admin.eq(user_data.is_admin),
            updated_at.eq(Utc::now()),
        ))
        .get_result(&mut read_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(updated_user.into()))
}

#[axum::debug_handler]
pub async fn delete(State(state): State<AppState>, Path(user_id): Path<i32>) -> StatusCode {
    use crate::schema::user::dsl::*;
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    diesel::delete(user.filter(id.eq(user_id)))
        .execute(&mut read_conn)
        .await
        .unwrap();

    StatusCode::OK
}
