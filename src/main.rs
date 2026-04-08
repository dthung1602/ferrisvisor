mod common;
mod handlers;
mod models;
mod schema;
mod supervisor;

use crate::common::AppState;
use axum::Router;
use axum::routing::{get, post, put};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

async fn ensure_admin_user(app_state: AppState) -> anyhow::Result<()> {
    use crate::models::NewUser;
    use diesel::QueryDsl;
    use diesel::dsl::{exists, select};
    use diesel::prelude::*;
    use schema::user::dsl::*;

    let mut conn = app_state.write_conn.get().await?;

    let new_user = NewUser::new("admin@example.com", "test123", true);

    let select_statement = select(exists(user.filter(email.eq(&new_user.email))));
    let admin_exist: bool =
        diesel_async::RunQueryDsl::get_result(select_statement, &mut conn).await?;

    if admin_exist {
        println!("Admin user already exists");
        return Ok(());
    }

    println!("Creating admin user {:?}", new_user);
    let insert_statement = diesel::insert_into(schema::user::table).values(&new_user);
    diesel_async::RunQueryDsl::execute(insert_statement, &mut conn).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("ferrisvisor=debug,tower_http=debug"))
                .expect("Can't set up logging"),
        )
        .init();

    let state = AppState::new().await?;

    // TODO only dev
    ensure_admin_user(state.clone()).await?;

    let admin_middleware = axum::middleware::from_fn(handlers::auth::admin_middleware);

    let user_perm_router = Router::new()
        .route(
            "/",
            post(handlers::permission::create).get(handlers::permission::list),
        )
        .route(
            "/{permission_id}",
            put(handlers::permission::update).delete(handlers::permission::delete),
        );

    let single_user_router = Router::new()
        .route(
            "/",
            put(handlers::user::update)
                .delete(handlers::user::delete)
                .get(handlers::user::get)
                .route_layer(admin_middleware.clone()),
        )
        .nest("/permission", user_perm_router);

    let api_router = Router::new()
        .route(
            "/host",
            post(handlers::host::create)
                .route_layer(admin_middleware.clone())
                .get(handlers::host::list),
        )
        .route(
            "/host/{host_id}",
            put(handlers::host::update)
                .delete(handlers::host::delete)
                .route_layer(admin_middleware.clone())
                .get(handlers::host::get),
        )
        .route(
            "/group",
            post(handlers::group::create)
                .route_layer(admin_middleware.clone())
                .get(handlers::group::list),
        )
        .route(
            "/group/{group_id}",
            put(handlers::group::update)
                .delete(handlers::group::delete)
                .route_layer(admin_middleware.clone())
                .get(handlers::group::get),
        )
        .route(
            "/user",
            post(handlers::user::create)
                .get(handlers::user::list)
                .route_layer(admin_middleware.clone()),
        )
        .nest("/user/{user_id}", single_user_router)
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            handlers::auth::auth_middleware,
        ))
        .route("/login", post(handlers::auth::login))
        .route("/logout", post(handlers::auth::logout))
        .route("/me", get(handlers::auth::get_current_user_with_permission))
        .with_state(state);

    let router = Router::new()
        .nest("/api", api_router)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Cannot bind to port 3000");
    axum::serve(listener, router)
        .await
        .expect("Cannot start server");

    Ok(())
}
