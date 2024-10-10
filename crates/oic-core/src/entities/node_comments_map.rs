//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "node_comments_map")]
pub struct Model {
    pub bundle: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub nid: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub comment_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}