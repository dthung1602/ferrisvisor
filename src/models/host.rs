use crate::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::host)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Host {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
    pub hostname: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = schema::host)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewHost {
    pub group_id: i32,
    pub name: String,
    pub hostname: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}
