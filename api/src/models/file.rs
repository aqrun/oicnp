use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct File {
    pub fid: i32,
    pub uid: i32,
    pub filename: String,
    pub uri: String,
    pub storage: String,
    pub mime: String,
    pub site: i32,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "file"]
pub struct NewFile {
    pub filename: String,
    pub uri: String,
    pub storage: String,
    pub mime: String,
    pub site: i32,
    pub status: i32,
}