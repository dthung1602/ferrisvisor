use crate::schema;
use crate::supervisor::{ProcessConfig, ProcessInfo};
use argon2::PasswordVerifier;
use argon2::password_hash::SaltString;
use chrono::{DateTime, Duration, TimeDelta, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ------------------------------------------
// region group
// ------------------------------------------

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

// endregion

// ------------------------------------------
// region host
// ------------------------------------------

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

// endregion

// ------------------------------------------
// region user
// ------------------------------------------

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

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
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

#[derive(Insertable, Debug, Deserialize)]
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
pub struct UpdateUser {
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DisplayUser {
    pub id: i32,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_admin: bool,
}

impl DisplayUser {
    pub fn from_user(user: &User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            last_login: user.last_login,
            is_admin: user.is_admin,
        }
    }
}

impl Into<DisplayUser> for User {
    fn into(self) -> DisplayUser {
        DisplayUser::from_user(&self)
    }
}

// endregion

// ------------------------------------------
// region login
// ------------------------------------------

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

// endregion

// ------------------------------------------
// region permission
// ------------------------------------------

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = schema::permission)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Permission {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub host_id: Option<i32>,
    pub service_name: String,
    pub can_view: bool,
    pub can_act: bool,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = schema::permission)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewPermission {
    pub user_id: i32,
    pub group_id: i32,
    pub host_id: Option<i32>,
    pub service_name: String,
    pub can_view: bool,
    pub can_act: bool,
}

impl NewPermission {
    pub fn new(
        user_id: i32,
        group_id: i32,
        host_id: Option<i32>,
        service_name: &str,
        can_view: bool,
        can_act: bool,
    ) -> Self {
        Self {
            user_id,
            group_id,
            host_id,
            service_name: service_name.to_string(),
            can_view: can_view | can_act, // Can act implies can view
            can_act,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdatePermission {
    pub host_id: Option<i32>,
    pub group_id: i32,
    pub service_name: String,
    pub can_view: bool,
    pub can_act: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct DisplayPermission {
    pub id: i32,
    pub user_id: i32,
    pub host_id: Option<i32>,
    pub host_name: Option<String>,
    pub group_id: i32,
    pub group_name: String,
    pub service_name: String,
    pub can_view: bool,
    pub can_act: bool,
}

impl DisplayPermission {
    pub fn from_perm(perm: Permission, host_name: Option<String>, group_name: String) -> Self {
        Self {
            host_name,
            group_name,
            id: perm.id,
            user_id: perm.user_id,
            host_id: perm.host_id,
            group_id: perm.group_id,
            service_name: perm.service_name.clone(),
            can_view: perm.can_view,
            can_act: perm.can_act,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct UserWithPermissions {
    pub id: i32,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_admin: bool,
    pub session: Session,
    pub permissions: Vec<DisplayPermission>,
}

// endregion

// ------------------------------------------
// region process
// ------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessQuery {
    pub group_id: Option<i32>,
    pub host_id: Option<i32>,
    pub process_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DisplayProcess {
    pub group_id: i32,
    pub host_id: i32,
    pub process: ProcessInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessConfigQuery {
    pub host_id: i32,
    pub process_name: Option<String>,
}

#[derive(Debug, Serialize)]
    pub struct DisplayProcessConfig {
    pub host_id: i32,
    pub config: ProcessConfig
}

// endregion
