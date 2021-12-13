use async_graphql::{Object};
use serde::{Serialize, Deserialize};
use rbatis::DateTimeNative;

#[crud_table]
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl Default for File {
    fn default() -> Self {
        Self {
            fid: 0,
            uid: 0,
            filename: "".to_string(),
            uri: "".to_string(),
            storage: "".to_string(),
            mime: "".to_string(),
            site: 0,
            status: 0,
            created_at: DateTimeNative::now(),
            updated_at: DateTimeNative::now(),
        }
    }
}

#[Object]
impl File {
    async fn fid(&self) -> i32 {
        self.fid
    }
    async fn filename(&self) -> &str {
        self.filename.as_str()
    }
    async fn uri(&self) -> &str {
        self.uri.as_str()
    }
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
