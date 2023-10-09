use crate::DateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct NewUser {
    pub uid: Option<String>,
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

#[derive(Clone, Debug, Default)]
pub struct UpdateUser {
    pub uid: String,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub status: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub role_id: Option<String>,
    pub department_id: Option<String>,
    pub remark: Option<String>,
    pub is_admin: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_at: Option<DateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: Option<DateTime>,
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
