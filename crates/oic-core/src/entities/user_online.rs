//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "user_online")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid: i64,
    pub token_id: String,
    pub token_expire: i64,
    pub login_at: DateTime,
    pub username: String,
    pub dpt_name: String,
    pub net: String,
    pub ip: String,
    pub location: String,
    pub device: String,
    pub browser: String,
    pub os: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
