//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "user_department_map")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub dpt_id: i64,
    pub created_by: Option<i64>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
