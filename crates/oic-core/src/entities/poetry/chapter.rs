use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 诗词章节表
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "chapters")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: String,
    #[serde(rename(deserialize = "pid", serialize = "pid"))]
    pub pid: i32,
    #[serde(rename(deserialize = "poetryId", serialize = "poetryId"))]
    pub poetry_id: i32,
    pub title: String,
    /// 章节说明
    pub description: String,
    /// 章节总内容
    pub content: String,
    #[serde(rename(deserialize = "wordCount", serialize = "wordCount"))]
    pub word_count: i16,
    pub weight: i16,
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
    #[sea_orm(has_many = "super::chapter_line::Entity")]
    ChapterLine,
}

impl Related<super::chapter_line::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChapterLine.def()
    }
}

impl Related<super::poetry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Poetry.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}