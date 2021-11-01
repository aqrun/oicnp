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

#[derive(Queryable)]
pub struct NodeBody {
    pub nid: i32,
    pub summary: String,
    pub body: String,
    pub body_format: String,
}

