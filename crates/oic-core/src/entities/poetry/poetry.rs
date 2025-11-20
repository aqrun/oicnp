use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 诗词表
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "poetry")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: String,
    pub title: String,
    #[serde(rename(deserialize = "authorId", serialize = "authorId"))]
    pub author_id: i32,
    pub dynasty: String,
    pub weight: i32,
    #[serde(rename(deserialize = "hotWeight", serialize = "hotWeight"))]
    pub hot_weight: i16,
    pub content: String,
    #[serde(rename(deserialize = "wordCount", serialize = "wordCount"))]
    pub word_count: i16,
    pub tags: String,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::poetry_line::Entity")]
    PoetryLine,
    #[sea_orm(has_many = "super::chapter::Entity")]
    Chapter,
}

impl Related<super::poetry_line::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoetryLine.def()
    }
}

impl Related<super::chapter::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chapter.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}