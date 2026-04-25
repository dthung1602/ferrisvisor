use crate::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::group)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub color: String,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = schema::group)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewGroup {
    pub name: String,
    pub description: String,
    pub color: String,
}
