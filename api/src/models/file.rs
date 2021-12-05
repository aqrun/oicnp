// use chrono::NaiveDateTime;

#[crud_table]
#[derive(Clone, Debug)]
pub struct File {
    pub fid: Option<i32>,
    pub uid: Option<i32>,
    pub filename: Option<String>,
    pub uri: Option<String>,
    pub storage: Option<String>,
    pub mime: Option<String>,
    pub site: Option<i32>,
    pub status: Option<i32>,
    pub created_at: Option<rbatis::DateTimeNative>,
    pub updated_at: Option<rbatis::DateTimeNative>,
}
