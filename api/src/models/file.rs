
#[crud_table]
#[derive(Clone, Debug)]
pub struct File {
    pub fid: i32,
    pub uid: i32,
    pub filename: String,
    pub uri: String,
    pub storage: String,
    pub mime: String,
    pub site: i32,
    pub status: i32,
    pub created_at: rbatis::DateTimeNative,
    pub updated_at: rbatis::DateTimeNative,
}

#[crud_table(table_name: file)]
#[derive(Clone, Debug)]
pub struct NewFile {
    pub uid: i32,
    pub filename: String,
    pub uri: String,
    pub storage: String,
    pub mime: String,
    pub site: i32,
    pub status: i32,
}
