//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cms_user_files_map")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub fid: String,
    pub bundle: Option<String>,
    pub weight: Option<i32>,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
