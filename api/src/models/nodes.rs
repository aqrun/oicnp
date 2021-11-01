use chrono::NaiveDateTime;

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

#[derive(Queryable, Insertable)]
pub struct NodeBody {
    pub nid: i32,
    pub summary: String,
    pub body: String,
    pub body_format: String,
}

#[derive(Queryable, Insertable)]
pub struct NodeCategoryMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

#[derive(Queryable, Insertable)]
pub struct NodeCommentsMap {
    pub bundle: String,
    pub nid: i32,
    pub cid: i32,
}

#[derive(Queryable, Insertable)]
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
pub struct NodeTagsMap {
    pub bundle: String,
    pub nid: i32,
    pub tid: i32,
}

