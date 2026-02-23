use crate::common::AppState;
use crate::models::{AuthenticatedUser, HasPassword, LoginForm, NewSession, User};
use crate::schema;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::cookie::CookieJar;
use diesel::QueryDsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::log::__private_api::log;

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(login_form): Json<LoginForm>,
) -> (StatusCode, Json<Option<AuthenticatedUser>>) {
    let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");

    let user_obj: Option<User> = schema::user::table
        .filter(schema::user::email.eq(&login_form.email))
        .first(&mut read_conn)
        .await
        .optional()
        .unwrap();
    println!("User {:?}", user_obj);
    drop(read_conn);

    let Some(user_obj) = user_obj else {
        return (StatusCode::UNAUTHORIZED, Json(None));
    };

    if !user_obj.verify_password(login_form.password.as_str()) {
        return (StatusCode::UNAUTHORIZED, Json(None));
    }

    let mut write_conn = state.write_conn.get().await.expect("Cannot get db conn");

    let session_obj = NewSession::new(user_obj.id);
    diesel::insert_into(schema::session::table)
        .values(&session_obj)
        .execute(&mut write_conn)
        .await
        .unwrap();

    drop(write_conn);

    let user_obj = AuthenticatedUser {
        email: user_obj.email,
        is_admin: user_obj.is_admin,
        token: session_obj.token,
        expires_at: session_obj.expires_at,
    };

    (StatusCode::OK, Json(Some(user_obj)))
}

const LOGIN_COOKIE_NAME: &'static str = "session_token";

#[axum::debug_handler]
pub async fn logout(State(state): State<AppState>, cookie_jar: CookieJar) {
    if let Some(login_cookie) = cookie_jar.get(LOGIN_COOKIE_NAME) {
        let mut read_conn = state.read_pool.get().await.expect("Cannot get db conn");
        let match_token = schema::session::token.eq(login_cookie.value());
        diesel::delete(schema::session::table.filter(match_token))
            .execute(&mut read_conn)
            .await
            .unwrap();
    }
}
