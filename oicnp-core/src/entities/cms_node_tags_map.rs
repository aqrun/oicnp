//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cms_node_tags_map")]
pub struct Model {
    pub bundle: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub nid: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}