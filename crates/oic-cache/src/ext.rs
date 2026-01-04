use crate::{Cache, CacheError, Result};
use crate::NamespaceInfo;
use crate::VaryCondition;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::future::Future;

/// 缓存扩展方法
/// 
/// 提供便捷的缓存方法，包括：
/// - Loco_rs 兼容的 API（自动序列化/反序列化）
/// - Axum 便捷方法（JSON、HTML 缓存）
#[allow(async_fn_in_trait)]
pub trait CacheExt {
    // ============ Loco_rs 兼容 API ============
    
    /// 检查缓存是否可达（ping）
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// cache.ping().await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn ping(&self) -> Result<()>;

    /// 检查键是否存在
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// let exists = cache.contains_key("key").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn contains_key(&self, key: &str) -> Result<bool>;

    /// 获取缓存并自动反序列化为指定类型
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// let user: Option<User> = cache.get("user:1").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>>;

    /// 插入序列化值到缓存
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// use serde::Serialize;
    /// 
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// let user = User { name: "Alice".to_string(), age: 30 };
    /// cache.insert("user:1", &user).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn insert<T: Serialize + Sync + ?Sized>(
        &self,
        key: &str,
        value: &T,
    ) -> Result<()>;

    /// 插入序列化值到缓存（带过期时间）
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// use serde::Serialize;
    /// use std::time::Duration;
    /// 
    /// #[derive(Serialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// let user = User { name: "Alice".to_string(), age: 30 };
    /// cache.insert_with_expiry("user:1", &user, Duration::from_secs(300)).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn insert_with_expiry<T: Serialize + Sync + ?Sized>(
        &self,
        key: &str,
        value: &T,
        duration: Duration,
    ) -> Result<()>;

    /// 获取缓存值，如果不存在则使用闭包生成并插入
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// let user = cache.get_or_insert("user:1", async {
    ///     Ok(User { name: "Alice".to_string(), age: 30 })
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_or_insert<T, F>(&self, key: &str, f: F) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync,
        F: Future<Output = Result<T>> + Send;

    /// 获取缓存值，如果不存在则使用闭包生成并插入（带过期时间）
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// use serde::{Serialize, Deserialize};
    /// use std::time::Duration;
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     name: String,
    ///     age: u32,
    /// }
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// let user = cache.get_or_insert_with_expiry("user:1", Duration::from_secs(300), async {
    ///     Ok(User { name: "Alice".to_string(), age: 30 })
    /// }).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_or_insert_with_expiry<T, F>(
        &self,
        key: &str,
        duration: Duration,
        f: F,
    ) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync,
        F: Future<Output = Result<T>> + Send;

    /// 删除缓存键
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// cache.remove("key").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn remove(&self, key: &str) -> Result<()>;

    /// 清空所有缓存
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// cache.clear().await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn clear(&self) -> Result<()>;

    // ============ Axum 便捷方法 ============
    
    /// 设置 JSON 缓存（自动序列化）
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// use serde::Serialize;
    /// 
    /// #[derive(Serialize)]
    /// struct User {
    ///     id: u64,
    ///     name: String,
    /// }
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// let user = User { id: 1, name: "Alice".to_string() };
    /// cache.set_json("user:1", &user, 3600).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn set_json<T: Serialize>(
        &self,
        key: String,
        value: &T,
        ttl_seconds: i64,
    ) -> Result<()>;

    /// 设置 HTML 缓存
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// let html = "<h1>Hello</h1>".to_string();
    /// cache.set_html("page:home", &html, 3600).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn set_html(
        &self,
        key: String,
        html: &str,
        ttl_seconds: i64,
    ) -> Result<()>;

    /// 获取 JSON 缓存（自动反序列化）
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// use serde::{Serialize, Deserialize};
    /// 
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     id: u64,
    ///     name: String,
    /// }
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// 
    /// if let Some(user) = cache.get_json::<User>("user:1").await? {
    ///     println!("User: {}", user.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>>;

    /// 获取 HTML 缓存
    /// 
    /// # 示例
    /// ```no_run
    /// use oic_cache::{Cache, CacheConfig, CacheExt};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let cache = Cache::new(CacheConfig::default());
    /// 
    /// if let Some(html) = cache.get_html("page:home").await? {
    ///     println!("HTML: {}", html);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    async fn get_html(&self, key: &str) -> Result<Option<String>>;

    /// 设置 JSON 缓存（带命名空间）
    async fn set_json_with_namespace<T: Serialize>(
        &self,
        key: String,
        value: &T,
        namespace: NamespaceInfo,
        ttl_seconds: i64,
    ) -> Result<()>;

    /// 设置 JSON 缓存（带 Vary）
    async fn set_json_with_vary<T: Serialize>(
        &self,
        key: String,
        value: &T,
        vary_conditions: Vec<VaryCondition>,
        ttl_seconds: i64,
    ) -> Result<()>;
}

impl CacheExt for Cache {
    // ============ Loco_rs 兼容 API ============
    
    async fn ping(&self) -> Result<()> {
        // 简单的健康检查：尝试获取一个不存在的键来验证缓存可用性
        let _ = self.exists("__ping_check__").await;
        Ok(())
    }

    async fn contains_key(&self, key: &str) -> Result<bool> {
        Ok(self.exists(key).await)
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        match Cache::get(self, key).await? {
            Some(bytes) => {
                let value: T = serde_json::from_slice(&bytes)
                    .map_err(|e| CacheError::Serialization(format!("Failed to deserialize JSON: {}", e)))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    async fn insert<T: Serialize + Sync + ?Sized>(
        &self,
        key: &str,
        value: &T,
    ) -> Result<()> {
        let serialized = serde_json::to_vec(value)
            .map_err(|e| CacheError::Serialization(format!("Failed to serialize JSON: {}", e)))?;
        
        Cache::set(self, key.to_string(), serialized, "application/json".to_string()).await
    }

    async fn insert_with_expiry<T: Serialize + Sync + ?Sized>(
        &self,
        key: &str,
        value: &T,
        duration: Duration,
    ) -> Result<()> {
        let serialized = serde_json::to_vec(value)
            .map_err(|e| CacheError::Serialization(format!("Failed to serialize JSON: {}", e)))?;
        
        let ttl_seconds = duration.as_secs() as i64;
        Cache::set_with_ttl(self, key.to_string(), serialized, "application/json".to_string(), ttl_seconds).await
    }

    async fn get_or_insert<T, F>(&self, key: &str, f: F) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync,
        F: Future<Output = Result<T>> + Send,
    {
        if let Some(value) = CacheExt::get::<T>(self, key).await? {
            Ok(value)
        } else {
            let value = f.await?;
            self.insert(key, &value).await?;
            Ok(value)
        }
    }

    async fn get_or_insert_with_expiry<T, F>(
        &self,
        key: &str,
        duration: Duration,
        f: F,
    ) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync,
        F: Future<Output = Result<T>> + Send,
    {
        if let Some(value) = CacheExt::get::<T>(self, key).await? {
            Ok(value)
        } else {
            let value = f.await?;
            self.insert_with_expiry(key, &value, duration).await?;
            Ok(value)
        }
    }

    async fn remove(&self, key: &str) -> Result<()> {
        Cache::invalidate(self, key).await
    }

    async fn clear(&self) -> Result<()> {
        Cache::clear(self).await
    }

    // ============ Axum 便捷方法 ============
    
    async fn set_json<T: Serialize>(
        &self,
        key: String,
        value: &T,
        ttl_seconds: i64,
    ) -> Result<()> {
        let json_bytes = serde_json::to_vec(value)
            .map_err(|e| CacheError::Serialization(format!("Failed to serialize JSON: {}", e)))?;
        
        Cache::set_with_ttl(self, key, json_bytes, "application/json".to_string(), ttl_seconds)
            .await
    }

    async fn set_html(
        &self,
        key: String,
        html: &str,
        ttl_seconds: i64,
    ) -> Result<()> {
        Cache::set_with_ttl(
            self,
            key,
            html.as_bytes().to_vec(),
            "text/html".to_string(),
            ttl_seconds,
        )
        .await
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>> {
        match Cache::get(self, key).await? {
            Some(bytes) => {
                let value: T = serde_json::from_slice(&bytes)
                    .map_err(|e| CacheError::Serialization(format!("Failed to deserialize JSON: {}", e)))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    async fn get_html(&self, key: &str) -> Result<Option<String>> {
        match Cache::get(self, key).await? {
            Some(bytes) => {
                let html = String::from_utf8(bytes)
                    .map_err(|e| CacheError::Serialization(format!("Invalid UTF-8 in HTML: {}", e)))?;
                Ok(Some(html))
            }
            None => Ok(None),
        }
    }

    async fn set_json_with_namespace<T: Serialize>(
        &self,
        key: String,
        value: &T,
        namespace: NamespaceInfo,
        _ttl_seconds: i64,
    ) -> Result<()> {
        let json_bytes = serde_json::to_vec(value)
            .map_err(|e| CacheError::Serialization(format!("Failed to serialize JSON: {}", e)))?;
        
        Cache::set_with_namespace(self, key, json_bytes, "application/json".to_string(), namespace)
            .await
    }

    async fn set_json_with_vary<T: Serialize>(
        &self,
        key: String,
        value: &T,
        vary_conditions: Vec<VaryCondition>,
        _ttl_seconds: i64,
    ) -> Result<()> {
        let json_bytes = serde_json::to_vec(value)
            .map_err(|e| CacheError::Serialization(format!("Failed to serialize JSON: {}", e)))?;
        
        // Note: set_with_vary doesn't take ttl_seconds, uses default TTL
        Cache::set_with_vary(self, key, json_bytes, "application/json".to_string(), vary_conditions)
            .await
    }
}

