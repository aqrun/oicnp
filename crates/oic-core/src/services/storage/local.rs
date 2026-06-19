use std::sync::Arc;

use bytes::Bytes;
use futures_util::TryStreamExt;
use opendal::services::Fs;
use opendal::Operator;

use crate::models::files::CreateFileReqParams;
use crate::services::settings::StorageSettings;

use super::error::StorageError;
use super::key::{build_object_key, join_public_url};
use super::{StorageListResult, StorageObject, StorageProvider};

pub struct LocalStorageProvider {
    op: Operator,
    base_uri: String,
    prefix: String,
}

impl LocalStorageProvider {
    pub fn new(cfg: &StorageSettings) -> Result<Self, StorageError> {
        if cfg.path.is_empty() {
            return Err(StorageError::Message(
                "local storage path is required".into(),
            ));
        }

        let mut builder = Fs::default();
        builder.root(&cfg.path);

        let op = Operator::new(builder)
            .map_err(|err| StorageError::Message(err.to_string()))?
            .finish();

        Ok(Self {
            op,
            base_uri: cfg.uri.clone(),
            prefix: cfg.prefix.clone(),
        })
    }
}

#[async_trait::async_trait]
impl StorageProvider for LocalStorageProvider {
    async fn store(
        &self,
        data: Bytes,
        params: &CreateFileReqParams,
    ) -> Result<String, StorageError> {
        let filename = params.filename.as_deref().unwrap_or("");
        let key = build_object_key(filename);

        self.op.write(&key, data).await?;

        Ok(key)
    }

    async fn delete(&self, uri: &str) -> Result<(), StorageError> {
        self.op.delete(uri).await?;
        Ok(())
    }

    async fn list(
        &self,
        prefix: Option<&str>,
        limit: u64,
        _marker: Option<&str>,
    ) -> Result<StorageListResult, StorageError> {
        let list_prefix = prefix.unwrap_or("");
        let mut lister = self.op.lister(list_prefix).await?;
        let mut objects = Vec::new();

        while let Some(entry) = lister.try_next().await? {
            if entry.metadata().is_file() {
                objects.push(StorageObject {
                    key: entry.path().to_string(),
                    size: entry.metadata().content_length(),
                });
            }

            if objects.len() as u64 >= limit {
                break;
            }
        }

        Ok(StorageListResult {
            objects,
            next_marker: None,
        })
    }

    fn public_url(&self, uri: &str) -> String {
        join_public_url(&self.base_uri, &self.prefix, uri)
    }
}

pub fn new_provider(cfg: &StorageSettings) -> Result<Arc<dyn StorageProvider>, StorageError> {
    Ok(Arc::new(LocalStorageProvider::new(cfg)?))
}
