//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub uid: i64,
    pub uuid: String,
    #[sea_orm(default_value = "")]
    pub username: String,
    #[sea_orm(default_value = "")]
    pub nickname: String,
    pub password: String,
    #[sea_orm(default_value = "")]
    pub salt: String,
    #[serde(rename(deserialize = "apiKey", serialize = "apiKey"))]
    #[sea_orm(default_value = "")]
    pub api_key: String,
    #[serde(rename(deserialize = "resetToken", serialize = "resetToken"))]
    #[sea_orm(default_value = "")]
    pub reset_token: String,
    #[serde(rename(deserialize = "resetSentAt", serialize = "resetSentAt"))]
    pub reset_sent_at: Option<DateTime>,
    #[serde(rename(deserialize = "emailVerifyToken", serialize = "emailVerifyToken"))]
    #[sea_orm(default_value = "")]
    pub email_verify_token: String,
    #[serde(rename(deserialize = "emailVerifySentAt", serialize = "emailVerifySentAt"))]
    pub email_verify_sent_at: Option<DateTime>,
    #[serde(rename(deserialize = "emailVerifiedAt", serialize = "emailVerifiedAt"))]
    pub email_verified_at: Option<DateTime>,
    pub status: String,
    #[sea_orm(default_value = "")]
    pub email: String,
    #[sea_orm(default_value = "")]
    pub gender: String,
    #[sea_orm(default_value = "")]
    pub avatar: String,
    #[serde(rename(deserialize = "roleId", serialize = "roleId"))]
    #[sea_orm(default_value = 0)]
    pub role_id: i64,
    #[serde(rename(deserialize = "departmentId", serialize = "departmentId"))]
    #[sea_orm(default_value = 0)]
    pub dpt_id: i64,
    #[sea_orm(default_value = "")]
    pub remark: String,
    #[serde(rename(deserialize = "isAdmin", serialize = "isAdmin"))]
    #[sea_orm(default_value = "")]
    pub is_admin: String,
    #[sea_orm(default_value = "")]
    pub phone: String,
    #[serde(rename(deserialize = "lastLoginIp", serialize = "lastLoginIp"))]
    #[sea_orm(default_value = "")]
    pub last_login_ip: String,
    #[serde(rename(deserialize = "lastLoginAt", serialize = "lastLoginAt"))]
    pub last_login_at: Option<DateTime>,
    #[serde(rename(deserialize = "createdBy", serialize = "createdBy"))]
    #[sea_orm(default_value = 0)]
    pub created_by: i64,
    #[sea_orm(default_value = 0)]
    #[serde(rename(deserialize = "updatedBy", serialize = "updatedBy"))]
    pub updated_by: i64,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<DateTime>,
    #[serde(rename(deserialize = "deletedAt", serialize = "deletedAt"))]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_role_map::Entity")]
    UserRole,
}

impl Related<super::user_role_map::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRole.def()
    }
}
