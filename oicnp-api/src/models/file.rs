use async_graphql::{Object};
use serde::{Serialize, Deserialize};
use oicnp_core::{DateTime, DatabaseConnection};
use chrono::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Files {
    pub fid: i32,
    pub uid: i32,
    pub filename: String,
    pub uri: String,
    pub storage: String,
    pub mime: String,
    pub site: i32,
    pub status: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for Files {
    fn default() -> Self {
        let now = Local::now().naive_local();

        Self {
            fid: 0,
            uid: 0,
            filename: "".to_string(),
            uri: "".to_string(),
            storage: "".to_string(),
            mime: "".to_string(),
            site: 0,
            status: 0,
            created_at: now,
            updated_at: now,
        }
    }
}

#[Object]
impl Files {
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
