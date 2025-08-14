use std::time::Duration;
use std::sync::Arc;
use anyhow::Result;
use loco_rs::{
    prelude::*,
    cache::Cache,
};
use crate::entities::prelude::*;
use crate::prelude::ModelCrudHandler;
use crate::models::caches::{CreateCacheReqParams, CacheScope};
use crate::{utils::utc_now, constants::DATE_TIME_FORMAT};
use serde::{de::DeserializeOwned, Serialize};
use anyhow::anyhow;

pub struct OicCache {
    pub db: DatabaseConnection,
    pub cache: Arc<Cache>,
}

impl OicCache {
    pub fn new(
        db: DatabaseConnection,
        cache: Arc<Cache>,
    ) -> Self {
        Self {
            db,
            cache,
        }
    }

    ///
    /// 优先获取内存缓存数据
    /// 再获取数据表数据
    /// 
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let exist = self.cache.contains_key(key).await?;

        if exist {
            return Ok(self.cache.get::<T>(key).await?);
        }

        let res = CacheModel::find_by_vid(&self.db, key).await?;

        // 检测缓存过期
        if let Some(expired_at) = res.expired_at {
            if expired_at < utc_now() {
                return Ok(None);
            }
        }

        let value = res.cache_value;

        let deserialized = serde_json::from_str::<T>(&value)
            .map_err(|e| anyhow!("缓存数据反序列化失败: {}", e))?;

        Ok(Some(deserialized))
    }

    ///
    /// 插入缓存数据
    /// 
    pub async fn insert<T: Serialize + Sync + ?Sized>(
        &self,
        key: &str,
        value: &T,
    ) -> Result<()> {
        self.cache.insert(key, value).await?;

        let serialized = serde_json::to_string(value)
            .map_err(|e| {
                anyhow!("缓存数据序列化失败: {}", e)
            })?;

        let scope = Self::parse_scope_by_key(key);
        
        let create_model = CreateCacheReqParams {
            cache_key: Some(key.to_string()),
            cache_value: Some(serialized),
            scope: Some(scope.to_string()),
            created_at: Some(utc_now().format(DATE_TIME_FORMAT).to_string()),
            ..Default::default()
        };
        CacheModel::create(&self.db, &create_model).await?;

        Ok(())
    }

    ///
    /// 移除指定key
    /// 
    pub async fn remove(&self, key: &str) -> Result<()> {
        self.cache.remove(key).await?;
        CacheModel::delete_by_key(&self.db, key).await?;

        Ok(())
    }

    pub async fn clear(&self) -> Result<()> {
        self.cache.clear().await?;
        CacheModel::delete_all(&self.db).await?;

        Ok(())
    }

    pub async fn insert_with_expiry<T: Serialize + Sync + ?Sized>(
        &self,
        key: &str,
        value: &T,
        duration: Duration,
    ) -> Result<()> {
        self.cache.insert_with_expiry(key, value, duration).await?;
        
        let serialized = serde_json::to_string(value)
        .map_err(|e| {
            anyhow!("缓存数据序列化失败: {}", e)
        })?;

        let expired_at = utc_now() + duration;
        let mut scope = CacheScope::Other;
        
        if key.starts_with("session-") {
            scope = CacheScope::Session;
        } else if key.starts_with("captcha-") {
            scope = CacheScope::Captcha;
        }
        
        let create_model = CreateCacheReqParams {
            cache_key: Some(key.to_string()),
            cache_value: Some(serialized),
            scope: Some(scope.to_string()),
            created_at: Some(utc_now().format(DATE_TIME_FORMAT).to_string()),
            expired_at: Some(expired_at.format(DATE_TIME_FORMAT).to_string()),
            ..Default::default()
        };
        CacheModel::create(&self.db, &create_model).await?;

        Ok(())
    }

    ///
    /// 数据刷新
    /// 清空全部过期数据
    /// 
    pub async fn refresh(&self) -> Result<()> {
        CacheModel::refresh(&self.db).await?;

        Ok(())
    }

    ///
    /// 根据 key 解析缓存作用域
    /// 
    pub fn parse_scope_by_key(key: &str) -> CacheScope {
        if key.is_empty() {
            return CacheScope::Other;
        }

        if key.starts_with("captcha-") {
            return CacheScope::Captcha;
        }

        CacheScope::Other
    }
}
