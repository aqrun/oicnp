mod error;
pub mod key;
mod local;
mod oss;

pub use error::StorageError;
pub use key::{build_object_key, join_public_url};

use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;

use crate::models::files::CreateFileReqParams;
use crate::services::settings::StorageSettings;

#[derive(Debug, Clone)]
pub struct StorageObject {
    pub key: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct StorageListResult {
    pub objects: Vec<StorageObject>,
    pub next_marker: Option<String>,
}

#[async_trait]
pub trait StorageProvider: Send + Sync {
    async fn store(
        &self,
        data: Bytes,
        params: &CreateFileReqParams,
    ) -> Result<String, StorageError>;

    async fn delete(&self, uri: &str) -> Result<(), StorageError>;

    async fn list(
        &self,
        prefix: Option<&str>,
        limit: u64,
        marker: Option<&str>,
    ) -> Result<StorageListResult, StorageError>;

    fn public_url(&self, uri: &str) -> String;
}

pub struct StorageProviderFactory;

impl StorageProviderFactory {
    pub fn from_settings(cfg: &StorageSettings) -> Result<Arc<dyn StorageProvider>, StorageError> {
        Self::for_driver(cfg, cfg.driver.as_str())
    }

    pub fn for_driver(
        cfg: &StorageSettings,
        driver: &str,
    ) -> Result<Arc<dyn StorageProvider>, StorageError> {
        match driver {
            "local" => local::new_provider(cfg),
            "oss" => oss::new_provider(cfg),
            other => Err(StorageError::Message(format!(
                "unsupported storage driver: {other}"
            ))),
        }
    }
}
