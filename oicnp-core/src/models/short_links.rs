use crate::DateTime;
use serde::{Deserialize, Serialize};
use sea_orm::FromQueryResult;

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
pub struct ShortLink {
    pub id: String,
    pub link: String,
    pub name: String,
    pub description: String,
    pub viewed: i32,
    pub deleted: String,
    pub created_at: DateTime,
    pub created_by: String,
}

#[derive(Clone, Debug)]
pub struct NewShortLink {
    pub link: String,
    pub name: String,
    pub description: String,
    pub deleted: String,
    pub created_by: i64,
}