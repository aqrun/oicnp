use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 诗词作者表
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "authors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub description: String,
    #[serde(rename(deserialize = "shortDescription", serialize = "shortDescription"))]
    pub short_description: String,
    #[serde(rename(deserialize = "birthAt", serialize = "birthAt"))]
    pub birth_at: Option<DateTime>,
    #[serde(rename(deserialize = "deathAt", serialize = "deathAt"))]
    pub death_at: Option<DateTime>,
    pub dynasty: String,
    // small integer
    pub weight: i16,
    // 作品数量
    pub count: i16,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}