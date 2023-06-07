use crate::{DateTime,
    entities::{
        cms_node_taxonomies_map,
    },
};
use serde::{Deserialize, Serialize};
use sea_orm::FromQueryResult;

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
pub struct Node {
    pub nid: String,
    pub vid: String,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: String,
    pub published_at: Option<DateTime>,
    pub created_at: DateTime,
    pub created_by: String,
    pub updated_at: DateTime,
    pub updated_by: String,
    pub deleted_at: Option<DateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetailNode {
    pub nid: String,
    pub vid: String,
    pub uid: String,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: String,
    pub created_at: DateTime,
    pub created_by: String,
    pub updated_at: DateTime,
    pub updated_by: String,
    pub created_by_username: String,
    pub created_by_nickname: String,
    pub updated_by_username: String,
    pub updated_by_nickname: String,

    pub tid: String,
    pub category_bundle: String,
    pub category_name: String,
    pub category_vid: String,

    pub author_uid: String,
    pub author_username: String,
    pub author_nickname: String,

    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[derive(Clone, Debug)]
pub struct NewNode {
    pub vid: String,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub published_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
pub struct NodeBody {
    pub nid: String,
    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[derive(Clone, Debug)]
pub struct NodeTaxonomiesMap {
    pub bundle: String,
    pub nid: String,
    pub tid: String,
}

impl NodeTaxonomiesMap {
    pub fn from_model(model: &cms_node_taxonomies_map::Model) -> Self {
        Self {
            bundle: model.clone().bundle.unwrap_or("".to_string()),
            nid: model.clone().nid,
            tid: model.clone().tid,
        }
    }
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

#[derive(Clone, Debug, FromQueryResult)]
pub struct NodeCount {
    pub count: i32,
}