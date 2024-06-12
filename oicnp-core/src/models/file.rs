use oicnp_core::prelude::async_graphql::{self, Object};
use oicnp_core::{
    DateTime,
    prelude::{
        anyhow::{anyhow, Result},
        chrono::prelude::*,
    }
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Files {
    pub fid: String,
    pub uid: String,
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
            fid: "".to_string(),
            uid: "".to_string(),
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
    async fn fid(&self) -> &str {
        self.fid.as_str()
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
    pub uid: String,
    pub filename: String,
    pub uri: String,
    pub storage: String,
    pub mime: String,
    pub site: i32,
    pub status: i32,
}
