use chrono::{DateTime, Utc};
use diesel::prelude::*;
use crate::schema;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = schema::host)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Host {
    pub name: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct User {
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_admin: bool,
}

pub struct Session {
    pub user_id: i32,
    pub token: String,
    pub expires: DateTime<Utc>,
}

pub struct Permission {
    pub user_id: i32,
    pub host_id: i32,
    pub service_name: String,
    pub can_view: bool,
    pub can_act: bool,
}
