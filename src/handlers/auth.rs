use crate::common::AppState;
use crate::models::{HasPassword, LoginForm, NewSession, Session, User, UserWithPermissions};
use crate::schema;

use crate::handlers::permission::get_display_permissions;
use axum::Json;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::CookieJar;
use chrono::Utc;
use diesel::QueryDsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(login_form): Json<LoginForm>,
) -> (StatusCode, Json<Option<UserWithPermissions>>) {
    let mut db_conn = state.db_conn.lock().await;
    let now = Utc::now();

    let user: Option<User> = schema::user::table
        .filter(schema::user::email.eq(&login_form.email))
        .first(&mut db_conn)
        .await
        .optional()
        .unwrap();

    let Some(user) = user else {
        return (StatusCode::UNAUTHORIZED, Json(None));
    };

    if !user.verify_password(login_form.password.as_str()) {
        return (StatusCode::UNAUTHORIZED, Json(None));
    }

    let session = NewSession::new(user.id);
    let session = diesel::insert_into(schema::session::table)
        .values(&session)
        .get_result::<Session>(&mut db_conn)
        .await
        .unwrap();

    diesel::update(schema::user::table.filter(schema::user::id.eq(user.id)))
        .set(schema::user::last_login.eq(now))
        .execute(&mut db_conn)
        .await
        .unwrap();

    let permissions = get_display_permissions(user.id, None, &mut db_conn).await;

    let user_with_perms = UserWithPermissions {
        id: user.id,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
        last_login: Some(now),
        is_admin: user.is_admin,
        session: session.clone(),
        permissions,
    };

    (StatusCode::OK, Json(Some(user_with_perms)))
}

const LOGIN_COOKIE_NAME: &'static str = "session_token";

#[axum::debug_handler]
pub async fn logout(State(state): State<AppState>, cookie_jar: CookieJar) {
    if let Some(login_cookie) = cookie_jar.get(LOGIN_COOKIE_NAME) {
        let mut db_conn = state.db_conn.lock().await;
        let match_token = schema::session::token.eq(login_cookie.value());
        diesel::delete(schema::session::table.filter(match_token))
            .execute(&mut db_conn)
            .await
            .unwrap();
    }
}

#[axum::debug_handler]
pub async fn get_current_user_with_permission(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
) -> (StatusCode, Json<Option<UserWithPermissions>>) {
    let Some(login_cookie) = cookie_jar.get(LOGIN_COOKIE_NAME) else {
        return (StatusCode::UNAUTHORIZED, Json(None));
    };

    let mut db_conn = state.db_conn.lock().await;
    let match_token = schema::session::token.eq(login_cookie.value());
    let res = schema::session::table
        .filter(match_token)
        .left_join(schema::user::table)
        .select((Session::as_select(), Option::<User>::as_select()))
        .load::<(Session, Option<User>)>(&mut db_conn)
        .await
        .unwrap();

    if res.len() == 0 {
        return (StatusCode::UNAUTHORIZED, Json(None));
    }

    let (session, user) = &res[0];

    let Some(user) = user else {
        return (StatusCode::UNAUTHORIZED, Json(None));
    };

    let permissions = get_display_permissions(user.id, None, &mut db_conn).await;

    let user_with_perms = UserWithPermissions {
        id: user.id,
        email: user.email.clone(),
        created_at: user.created_at,
        updated_at: user.updated_at,
        last_login: user.last_login,
        is_admin: user.is_admin,
        session: session.clone(),
        permissions,
    };

    (StatusCode::OK, Json(Some(user_with_perms)))
}

pub async fn auth_middleware(
    state: State<AppState>,
    cookie_jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let (status_code, Json(user_with_perms)) =
        get_current_user_with_permission(state, cookie_jar).await;
    if status_code != StatusCode::OK {
        return (status_code, Json(user_with_perms)).into_response();
    }
    request.extensions_mut().insert(user_with_perms.unwrap());
    next.run(request).await
}

pub async fn admin_middleware(request: Request, next: Next) -> Response {
    let user_with_perms = request.extensions().get::<UserWithPermissions>();
    if let Some(user_with_perms) = user_with_perms {
        if user_with_perms.is_admin {
            next.run(request).await
        } else {
            StatusCode::FORBIDDEN.into_response()
        }
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
