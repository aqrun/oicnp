//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::{default_string, default_i64, default_date_time};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub uid: i64,
    pub uuid: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub username: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub nickname: String,
    pub password: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub salt: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub api_key: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub reset_token: String,
    pub reset_sent_at: Option<DateTime>,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub email_verify_token: String,
    pub email_verify_sent_at: Option<DateTime>,
    pub email_verified_at: Option<DateTime>,
    pub status: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub email: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub gender: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub avatar: String,
    #[sea_orm(default_value = 0)]
    #[serde(default = "default_i64")]
    pub role_id: i64,
    #[sea_orm(default_value = 0)]
    #[serde(default = "default_i64")]
    pub dpt_id: i64,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub remark: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub is_admin: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub phone: String,
    #[sea_orm(default_value = "")]
    #[serde(default = "default_string")]
    pub last_login_ip: String,
    pub last_login_at: Option<DateTime>,
    #[sea_orm(default_value = 0)]
    #[serde(default = "default_i64")]
    pub created_by: i64,
    #[sea_orm(default_value = 0)]
    #[serde(default = "default_i64")]
    pub updated_by: i64,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    #[serde(default = "default_date_time")]
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
