use crate::models::session::Session;
use crate::schema;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
pub struct PermissionResponse {
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

impl PermissionResponse {
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
    pub permissions: Vec<PermissionResponse>,
}
