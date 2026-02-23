mod common;
mod handlers;
mod models;
mod schema;
mod supervisor;

use axum::Router;
use axum::routing::post;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use crate::common::AppState;

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

    let api_router = Router::new()
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout))
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


async fn ensure_admin_user(app_state: AppState) -> anyhow::Result<()> {
    use schema::user::dsl::*;
    use diesel::dsl::{select, exists};
    use diesel::prelude::*;
    use diesel::QueryDsl;
    use crate::models::NewUser;

    let mut conn = app_state.write_conn.get().await?;

    let new_user = NewUser::new("admin@example.com", "test123", true);

    let select_statement = select(exists(user.filter(email.eq(&new_user.email))));
    let admin_exist: bool = diesel_async::RunQueryDsl::get_result(select_statement, &mut conn).await?;

    if admin_exist {
        println!("Admin user already exists");
        return Ok(());
    }

    println!("Creating admin user {:?}", new_user);
    let insert_statement = diesel::insert_into(schema::user::table).values(&new_user);
    diesel_async::RunQueryDsl::execute(insert_statement, &mut conn).await?;

    Ok(())
}