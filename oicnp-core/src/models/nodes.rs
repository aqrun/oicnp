use crate::{DateTime};
use serde::{Deserialize, Serialize};
use sea_orm::FromQueryResult;

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
pub struct Node {
    pub nid: i32,
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: bool,
    pub created_at: DateTime,
    pub created_by: i32,
    pub updated_at: DateTime,
    pub updated_by: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetailNode {
    pub nid: i32,
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: bool,
    pub created_at: DateTime,
    pub created_by: i32,
    pub updated_at: DateTime,
    pub updated_by: i32,
    pub created_by_username: String,
    pub created_by_nickname: String,
    pub updated_by_username: String,
    pub updated_by_nickname: String,

    pub tid: i32,
    pub category_bundle: String,
    pub category_name: String,
    pub category_vid: String,

    pub author_uid: i32,
    pub author_username: String,
    pub author_nickname: String,

    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[derive(Clone, Debug)]
pub struct NewNode {
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: i32,
    pub updated_by: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
pub struct NodeBody {
    pub nid: i32,
    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[derive(Clone, Debug)]
pub struct NodeTaxonomiesMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

#[derive(Clone, Debug)]
pub struct NodeCommentsMap {
    pub bundle: String,
    pub nid: i32,
    pub cid: i64,
}

#[derive(Clone, Debug)]
pub struct NodeImagesMap {
    pub bundle: String,
    pub nid: i32,
    pub fid: i32,
    pub weight: i32,
    pub alt: String,
    pub title: String,
    pub width: i32,
    pub height: i32,
}