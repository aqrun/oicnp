use crate::DateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct NewUser {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub salt: String,
    pub status: String,
    pub email: String,
    pub gender: String,
    pub phone: String,
    pub avatar: Option<String>,
    pub role_id: Option<String>,
    pub department_id: Option<String>,
    pub remark: Option<String>,
    pub is_admin: String,
    pub last_login_ip: String,
    pub last_login_at: Option<DateTime>,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromQueryResult)]
pub struct User {
    pub uid: String,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub salt: String,
    pub status: String,
    pub email: String,
    pub gender: String,
    pub phone: String,
    pub avatar: Option<String>,
    pub role_id: Option<String>,
    pub department_id: Option<String>,
    pub remark: Option<String>,
    pub is_admin: String,
    pub last_login_ip: String,
    pub last_login_at: Option<DateTime>,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}
