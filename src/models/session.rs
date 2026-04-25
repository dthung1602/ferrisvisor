use crate::schema;
use argon2::password_hash::SaltString;
use chrono::{DateTime, Duration, TimeDelta, Utc};
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Debug, Serialize, Clone)]
#[diesel(table_name = schema::session)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::session)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSession {
    pub user_id: i32,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

const SESSION_EXPIRE_TIMEDELTA: TimeDelta = Duration::days(7);

impl NewSession {
    pub fn new(user_id: i32) -> Self {
        use rand_core::OsRng;

        let salt: SaltString = SaltString::generate(&mut OsRng);
        let expires_at = Utc::now() + SESSION_EXPIRE_TIMEDELTA;
        Self {
            user_id,
            token: salt.to_string(),
            expires_at,
        }
    }
}
