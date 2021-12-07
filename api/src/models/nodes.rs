// use chrono::NaiveDateTime;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Node {
    pub nid: i32,
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub created_at: rbatis::DateTimeNative,
    pub created_by: i32,
    pub updated_at: rbatis::DateTimeNative,
    pub updated_by: i32,
}

#[crud_table(table_name: node)]
#[derive(Clone, Debug)]
pub struct NewNode {
    pub vid: String,
    pub uid: i32,
    pub bundle: String,
    pub title: String,
    pub deleted: bool,
    pub created_by: i32,
    pub updated_by: i32,
}

#[crud_table(table_name: node_body)]
#[derive(Clone, Debug)]
pub struct NodeBody {
    pub nid: i32,
    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[crud_table]
#[derive(Clone, Debug)]
pub struct NodeCategoryMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

#[crud_table(table_name: node_comments_map)]
#[derive(Clone, Debug)]
pub struct NodeCommentsMap {
    pub bundle: String,
    pub nid: i32,
    pub cid: i64,
}

#[crud_table(table_name: node_images_map)]
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

#[crud_table(table_name: node_tags_map)]
#[derive(Clone, Debug)]
pub struct NodeTagsMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

