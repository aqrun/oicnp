//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "menus")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pid: String,
    pub path: String,
    pub name: String,
    pub icon: String,
    pub r#type: String,
    pub query: String,
    pub weight: i32,
    pub api: String,
    pub status: String,
    pub method: String,
    pub component: String,
    pub visible: String,
    pub is_cache: String,
    pub log_method: String,
    pub data_cache_method: String,
    pub is_frame: String,
    pub data_scope: String,
    pub remark: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
