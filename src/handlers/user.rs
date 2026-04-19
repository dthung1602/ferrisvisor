use crate::common::AppState;
use crate::models::{DisplayUser, HasPassword, NewUser, UpdateUser, User};
use crate::schema;
use diesel::QueryDsl;
use diesel::prelude::*;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use chrono::Utc;
use diesel_async::RunQueryDsl;

#[axum::debug_handler]
pub async fn list(State(state): State<AppState>) -> (StatusCode, Json<Vec<DisplayUser>>) {
    let mut db_conn = state.db_conn.lock().await;

    let users: Vec<User> = schema::user::table.load(&mut db_conn).await.unwrap();

    println!("Users {:?}", users);

    let users = users.iter().map(DisplayUser::from_user).collect();

    (StatusCode::OK, Json(users))
}

#[axum::debug_handler]
pub async fn get(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> (StatusCode, Json<DisplayUser>) {
    let mut db_conn = state.db_conn.lock().await;

    let user: User = schema::user::table
        .filter(schema::user::id.eq(user_id))
        .first(&mut db_conn)
        .await
        .unwrap();

    println!("Users {:?}", user);

    (StatusCode::OK, Json(user.into()))
}

#[axum::debug_handler]
pub async fn create(
    State(state): State<AppState>,
    Json(mut new_user): Json<NewUser>,
) -> (StatusCode, Json<DisplayUser>) {
    let mut db_conn = state.db_conn.lock().await;

    // hash password
    new_user.set_password(&new_user.password.clone());

    let user: User = diesel::insert_into(schema::user::table)
        .values(new_user)
        .get_result::<User>(&mut db_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(user.into()))
}

#[axum::debug_handler]
pub async fn update(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(user_data): Json<UpdateUser>,
) -> (StatusCode, Json<DisplayUser>) {
    use crate::schema::user::dsl::*;
    let mut db_conn = state.db_conn.lock().await;

    let updated_user: User = diesel::update(user.filter(id.eq(user_id)))
        .set((
            email.eq(user_data.email),
            is_admin.eq(user_data.is_admin),
            updated_at.eq(Utc::now()),
        ))
        .get_result(&mut db_conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(updated_user.into()))
}

#[axum::debug_handler]
pub async fn delete(State(state): State<AppState>, Path(user_id): Path<i32>) -> StatusCode {
    use crate::schema::user::dsl::*;
    let mut db_conn = state.db_conn.lock().await;

    diesel::delete(user.filter(id.eq(user_id)))
        .execute(&mut db_conn)
        .await
        .unwrap();

    StatusCode::OK
}
