use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::sync_connection_wrapper::SyncConnectionWrapper;
use diesel_async::AsyncConnection;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;


pub type AsyncSqliteConnection = SyncConnectionWrapper<SqliteConnection>;
pub type ReadConnObj = Object<AsyncDieselConnectionManager<AsyncSqliteConnection>>;

#[derive(Clone)]
pub struct Db {
    pub write_conn: Arc<AsyncSqliteConnection>,
    pub read_pool: Arc<Pool<AsyncSqliteConnection>>,
}

pub async fn establish_connection() -> anyhow::Result<Db> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = AsyncDieselConnectionManager::<AsyncSqliteConnection>::new(&database_url);
    let read_pool = Pool::builder(manager)
        .max_size(4)
        .build()
        .expect("Failed to create pool.");
    let write_conn = AsyncSqliteConnection::establish(&database_url).await?;
    Ok(Db {
        read_pool: Arc::new(read_pool),
        write_conn: Arc::new(write_conn),
    })
}
