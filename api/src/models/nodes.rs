use chrono::NaiveDateTime;
use crate::schema::{
    node, node_body, node_tags_map,
    node_category_map, node_images_map,
    node_comments_map,
};

#[derive(Queryable)]
pub struct Node {
    pub nid: i32,
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub created_by: i32,
    pub updated_at: NaiveDateTime,
    pub updated_by: i32,
}

#[derive(Insertable)]
#[table_name="node"]
pub struct NewNode {
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub created_by: i32,
    pub updated_at: NaiveDateTime,
    pub updated_by: i32,
}

#[derive(Queryable, Insertable)]
#[table_name="node_body"]
pub struct NodeBody {
    pub nid: i32,
    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[derive(Queryable, Insertable)]
#[table_name="node_category_map"]
pub struct NodeCategoryMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

#[derive(Queryable, Insertable)]
#[table_name="node_comments_map"]
pub struct NodeCommentsMap {
    pub bundle: String,
    pub nid: i32,
    pub cid: i64,
}

#[derive(Queryable, Insertable)]
#[table_name="node_images_map"]
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

#[derive(Queryable, Insertable)]
#[table_name="node_tags_map"]
pub struct NodeTagsMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

