use crate::schema;
use argon2::PasswordVerifier;
use argon2::password_hash::SaltString;
use chrono::{DateTime, Duration, TimeDelta, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = schema::host)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Host {
    pub id: i32,
    pub name: String,
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
    pub name: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub trait HasPassword {
    fn get_password(&self) -> &str;
    fn set_raw_password(&mut self, password: String);

    fn set_password(&mut self, password: &str) {
        use argon2::Argon2;
        use argon2::password_hash::{PasswordHasher, SaltString};
        use rand_core::OsRng;

        let password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let hashed_password = argon2.hash_password(password, &salt).unwrap().to_string();
        self.set_raw_password(hashed_password);
    }

    fn verify_password(&self, password: &str) -> bool {
        use argon2::Argon2;
        use argon2::password_hash::PasswordHash;

        let parsed_hash = PasswordHash::new(self.get_password()).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_admin: bool,
}

impl HasPassword for User {
    fn get_password(&self) -> &str {
        &self.password
    }
    fn set_raw_password(&mut self, password: String) {
        self.password = password;
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

impl HasPassword for NewUser {
    fn get_password(&self) -> &str {
        &self.password
    }
    fn set_raw_password(&mut self, password: String) {
        self.password = password;
    }
}

impl NewUser {
    pub fn new(email: &str, password: &str, is_admin: bool) -> Self {
        let mut user = NewUser {
            email: email.to_string(),
            password: "".to_string(),
            is_admin,
        };
        user.set_password(password);
        user
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthenticatedUser {
    pub email: String,
    pub is_admin: bool,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}


#[derive(Queryable, Selectable, Debug)]
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

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::permission)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Permission {
    pub id: i32,
    pub user_id: i32,
    pub host_id: i32,
    pub service_name: String,
    pub can_view: bool,
    pub can_act: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = schema::permission)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewPermission {
    pub user_id: i32,
    pub host_id: i32,
    pub service_name: String,
    pub can_view: bool,
    pub can_act: bool,
}

impl NewPermission {
    pub fn new(
        user_id: i32,
        host_id: i32,
        service_name: &str,
        can_view: bool,
        can_act: bool,
    ) -> Self {
        Self {
            user_id,
            host_id,
            service_name: service_name.to_string(),
            can_view: can_view | can_act,
            can_act,
        }
    }
}
