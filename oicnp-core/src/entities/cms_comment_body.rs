//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cms_comment_body")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub comment_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub body: Option<String>,
    pub body_format: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
