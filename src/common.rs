use diesel::prelude::*;
use diesel_async::AsyncConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub write_conn: Arc<Pool<AsyncSqliteConnection>>,
    pub read_pool: Arc<Pool<AsyncSqliteConnection>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let db = establish_connection().await?;
        Ok(Self {
            write_conn: db.write_conn,
            read_pool: db.read_pool,
        })
    }
}

pub type AsyncSqliteConnection = SyncConnectionWrapper<SqliteConnection>;
pub type ReadConnObj = Object<AsyncDieselConnectionManager<AsyncSqliteConnection>>;

#[derive(Clone)]
pub struct Db {
    pub write_conn: Arc<Pool<AsyncSqliteConnection>>,
    pub read_pool: Arc<Pool<AsyncSqliteConnection>>,
}

pub async fn establish_connection() -> anyhow::Result<Db> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let read_only_database_url = database_url.clone(); // TODO make this work properly

    let manager =
        AsyncDieselConnectionManager::<AsyncSqliteConnection>::new(&read_only_database_url);
    let read_pool = Pool::builder(manager)
        .max_size(4)
        .build()
        .expect("Failed to create pool.");

    let manager = AsyncDieselConnectionManager::<AsyncSqliteConnection>::new(&database_url);
    let write_conn = Pool::builder(manager)
        .max_size(1)
        .build()
        .expect("Failed to create pool.");

    Ok(Db {
        read_pool: Arc::new(read_pool),
        write_conn: Arc::new(write_conn),
    })
}
