use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_async::AsyncConnection;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type AsyncSqliteConnection = SyncConnectionWrapper<SqliteConnection>;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: Arc<Mutex<AsyncSqliteConnection>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let db_conn = Arc::new(Mutex::new(establish_connection().await?));
        Ok(Self { db_conn })
    }
}

pub async fn establish_connection() -> ConnectionResult<AsyncSqliteConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SyncConnectionWrapper::<SqliteConnection>::establish(&database_url).await
}
