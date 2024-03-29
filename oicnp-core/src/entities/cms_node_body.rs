//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cms_node_body")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub nid: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub summary: Option<String>,
    pub summary_format: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub body: Option<String>,
    pub body_format: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Node,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Node => Entity::belongs_to(super::cms_nodes::Entity)
                .from(Column::Nid)
                .to(super::cms_nodes::Column::Nid)
                .into(),
        }
    }
}

impl Related<super::cms_nodes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Node.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
