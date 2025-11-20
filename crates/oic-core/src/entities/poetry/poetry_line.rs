use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 诗词内容行表
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "poetry_lines")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[serde(rename(deserialize = "poetryId", serialize = "poetryId"))]
    pub poetry_id: i32,
    #[serde(rename(deserialize = "lineNumber", serialize = "lineNumber"))]
    pub line_number: i32,
    pub content: String,
    pub pinyin: String,
    /// 内容行说明 翻译
    pub description: String,
    /// 内容行注释 注解
    pub notes: String,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::poetry::Entity",
        from = "Column::PoetryId",
        to = "super::poetry::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Poetry,
}

impl Related<super::poetry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Poetry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}