//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, Default)]
#[sea_orm(table_name = "menus")]
#[serde(default)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub vid: String,
    pub pid: i64,
    pub name: String,
    pub path: String,
    pub depth: i32,
    pub p1: i64,
    pub p2: i64,
    pub p3: i64,
    pub p4: i64,
    pub p5: i64,
    pub p6: i64,
    pub p7: i64,
    pub p8: i64,
    pub icon: String,
    pub weight: i32,
    pub api: String,
    pub status: String,
    pub visible: String,
    #[serde(rename(deserialize = "isCache", serialize = "isCache"))]
    pub is_cache: String,
    #[serde(rename(deserialize = "isFrame", serialize = "isFrame"))]
    pub is_frame: String,
    pub remark: String,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<DateTime>,
    #[serde(rename(deserialize = "deletedAt", serialize = "deletedAt"))]
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
