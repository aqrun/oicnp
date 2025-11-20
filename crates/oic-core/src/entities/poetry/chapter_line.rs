use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 诗词章节内容行表
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "chapter_lines")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[serde(rename(deserialize = "chapterId", serialize = "chapterId"))]
    pub chapter_id: i32,
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
        belongs_to = "super::chapter::Entity",
        from = "Column::ChapterId",
        to = "super::chapter::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Chapter,
}

impl Related<super::chapter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chapter.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}