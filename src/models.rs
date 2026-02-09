use crate::schema;
use argon2::PasswordVerifier;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
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

#[derive(Insertable, Debug)]
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

pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expires: DateTime<Utc>,
}

pub struct Permission {
    pub id: i32,
    pub user_id: i32,
    pub host_id: i32,
    pub service_name: String,
    pub can_view: bool,
    pub can_act: bool,
}
