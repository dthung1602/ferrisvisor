use deadpool_diesel::Runtime;
use diesel::sqlite::SqliteConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_async::{AsyncConnection, SimpleAsyncConnection};
use dotenvy::dotenv;
use std::env;

pub type AsyncSqliteConnection = SyncConnectionWrapper<SqliteConnection>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<AsyncSqliteConnection>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let db_pool = establish_connection().await?;
        Ok(Self { db_pool })
    }
}

pub async fn establish_connection() -> anyhow::Result<Pool<AsyncSqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut config = ManagerConfig::<AsyncSqliteConnection>::default();
    config.custom_setup = Box::new(|conn_url| {
        Box::pin(async move {
            let mut conn = AsyncSqliteConnection::establish(&conn_url).await?;
            let sql = "
                PRAGMA journal_mode=WAL;
                PRAGMA busy_timeout=5000;
                PRAGMA foreign_keys=ON;
            ";
            conn.batch_execute(sql)
                .await
                .map_err(|e| diesel::ConnectionError::BadConnection(e.to_string()))?;
            Ok(conn)
        })
    });

    let manager = AsyncDieselConnectionManager::<AsyncSqliteConnection>::new_with_config(
        database_url,
        config,
    );

    let pool = Pool::builder(manager)
        .max_size(8)
        .runtime(Runtime::Tokio1)
        .build()?;

    Ok(pool)
}
