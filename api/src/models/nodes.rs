// use chrono::NaiveDateTime;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Node {
    pub nid: Option<i32>,
    pub vid: Option<String>,
    pub uid: Option<i32>,
    pub bundle: Option<String>,
    pub title: Option<String>,
    pub deleted: Option<bool>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub created_by: Option<i32>,
    pub updated_at: Option<rbatis::DateTimeNative>,
    pub updated_by: Option<i32>,
}

#[crud_table(table_name: node)]
#[derive(Clone, Debug)]
pub struct NodeBody {
    pub nid: Option<i32>,
    pub summary: Option<String>,
    pub body: Option<String>,
    pub body_format: Option<String>,
}

#[crud_table]
#[derive(Clone, Debug)]
pub struct NodeCategoryMap {
    pub bundle: Option<String>,
    pub nid: Option<i32>,
    pub tid: Option<i32>,
}

#[crud_table(table_name: node_comments_map)]
#[derive(Clone, Debug)]
pub struct NodeCommentsMap {
    pub bundle: Option<String>,
    pub nid: Option<i32>,
    pub cid: Option<i64>,
}

#[crud_table(table_name: node_images_map)]
#[derive(Clone, Debug)]
pub struct NodeImagesMap {
    pub bundle: Option<String>,
    pub nid: Option<i32>,
    pub fid: Option<i32>,
    pub weight: Option<i32>,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[crud_table(table_name: node_tags_map)]
#[derive(Clone, Debug)]
pub struct NodeTagsMap {
    pub bundle: Option<String>,
    pub nid: Option<i32>,
    pub tid: Option<i32>,
}

