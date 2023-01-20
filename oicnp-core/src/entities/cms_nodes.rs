//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cms_nodes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub nid: String,
    pub vid: Option<String>,
    pub bundle: Option<String>,
    pub title: Option<String>,
    pub viewed: Option<i32>,
    pub deleted: Option<String>,
    pub published_at: Option<DateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
