use crate::config::CacheConfig;
use crate::error::{CacheError, Result};
use crate::metadata::{
    CacheMetadata, Extensions, FetchStatus, NamespaceInfo,
    StorageInfo, StorageLocation, VaryCondition, VaryInfo,
};
use crate::stats::{CacheStatistics, Statistics, create_stats, update_stats};
use crate::storage::compression::{compress, decompress};
use crate::storage::file::{delete_file, read_file, write_file};
use crate::storage::inline::read_inline;
use crate::vary::{build_cache_key, generate_variant_key, VaryValues};
use crate::fetch::FetchProtection;
use crate::utils::create_content_info;
use dashmap::DashMap;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 缓存系统的公开 API
pub struct Cache {
    inner: Arc<CacheInner>,
}

struct CacheInner {
    /// 索引：内存中的元数据
    index: Arc<RwLock<LruCache<String, CacheMetadata>>>,
    
    /// 命名空间索引：namespace -> keys
    namespace_index: Arc<DashMap<String, Vec<String>>>,
    
    /// 标签索引：tag -> keys
    tag_index: Arc<DashMap<String, Vec<String>>>,
    
    /// Vary 索引：base_key -> VaryInfo
    vary_index: Arc<DashMap<String, VaryInfo>>,
    
    /// 配置
    config: CacheConfig,
    
    /// 磁盘路径
    disk_path: PathBuf,
    
    /// 统计信息
    stats: Statistics,
    
    /// 回源保护
    #[allow(dead_code)]
    fetch_protection: FetchProtection,
}

impl Cache {
    /// 创建新的缓存实例
    pub fn new(config: CacheConfig) -> Self {
        let capacity = NonZeroUsize::new(config.index_capacity).unwrap_or(NonZeroUsize::new(1000).unwrap());
        let disk_path = PathBuf::from(&config.disk_path);
        
        // 确保目录存在
        if let Err(e) = std::fs::create_dir_all(&disk_path) {
            tracing::warn!("Failed to create cache directory: {}", e);
        }
        
        Self {
            inner: Arc::new(CacheInner {
                index: Arc::new(RwLock::new(LruCache::new(capacity))),
                namespace_index: Arc::new(DashMap::new()),
                tag_index: Arc::new(DashMap::new()),
                vary_index: Arc::new(DashMap::new()),
                config: config.clone(),
                disk_path,
                stats: Statistics::new(),
                fetch_protection: FetchProtection::new(config.concurrency.fetch_timeout_seconds),
            }),
        }
    }
    
    /// 从配置文件加载
    pub async fn from_config_file(path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| CacheError::Io(e))?;
        let config: CacheConfig = toml::from_str(&content)
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to parse config: {}", e)))?;
        Ok(Self::new(config))
    }
    
    // ============ 基础操作 ============
    
    /// 获取缓存
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let start = std::time::Instant::now();
        
        // 从索引获取元数据
        let metadata = {
            let mut index = self.inner.index.write().await;
            index.get(key).cloned()
        };
        
        let metadata = match metadata {
            Some(m) => m,
            None => {
                self.inner.stats.record_miss();
                return Ok(None);
            }
        };
        
        // 检查是否过期
        if metadata.is_expired() {
            self.inner.stats.record_miss();
            // 异步删除过期项
            let _ = self.invalidate(key).await;
            return Ok(None);
        }
        
        // 检查是否有效
        if !metadata.is_valid() {
            self.inner.stats.record_miss();
            return Ok(None);
        }
        
        // 读取数据
        let data = match &metadata.storage.location {
            StorageLocation::Inline(_) => {
                read_inline(&metadata.storage.location)
                    .ok_or_else(|| CacheError::KeyNotFound(key.to_string()))?
            }
            StorageLocation::File(_) => {
                let mut data = read_file(&metadata.storage.location, &self.inner.disk_path)
                    .await?
                    .ok_or_else(|| CacheError::KeyNotFound(key.to_string()))?;
                
                // 解压数据
                if let Some(ref compression) = metadata.storage.compression {
                    data = decompress(&data, compression)?;
                }
                
                data
            }
        };
        
        // 更新统计
        let read_time_us = start.elapsed().as_micros() as u64;
        let mut updated_metadata = metadata.clone();
        if let Some(ref mut stats) = updated_metadata.extensions.stats.as_mut() {
            update_stats(stats, read_time_us, data.len() as u64);
        }
        
        // 更新索引中的元数据
        {
            let mut index = self.inner.index.write().await;
            index.put(key.to_string(), updated_metadata);
        }
        
        self.inner.stats.record_hit();
        Ok(Some(data))
    }
    
    /// 设置缓存（自动判断存储策略）
    pub async fn set(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
    ) -> Result<()> {
        self.set_with_ttl(key, data, content_type, self.inner.config.default_ttl_seconds).await
    }
    
    /// 设置缓存（指定 TTL）
    pub async fn set_with_ttl(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
        ttl_seconds: i64,
    ) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let size = data.len() as u64;
        
        // 决定存储策略
        let (storage_location, compression_info) = if size < self.inner.config.storage.inline_threshold {
            // 内联存储
            (StorageLocation::Inline(data.clone()), None)
        } else {
            // 文件存储
            let (final_data, compression) = if self.inner.config.compression.enabled
                && size >= self.inner.config.compression.min_size
            {
                let (compressed, comp_info) = compress(&data, self.inner.config.compression.default_algorithm)?;
                (compressed, Some(comp_info))
            } else {
                (data.clone(), None)
            };
            
            let file_path = write_file(&self.inner.disk_path, &key, &final_data).await?;
            (StorageLocation::File(file_path), compression)
        };
        
        // 创建元数据
        let metadata = CacheMetadata {
            version: 1,
            key: key.clone(),
            size,
            created_at: now,
            expires_at: now + ttl_seconds,
            fetch_status: FetchStatus::Success,
            last_fetch_attempt: now,
            storage: StorageInfo {
                location: storage_location,
                compression: compression_info,
            },
            content: create_content_info(content_type, &data),
            extensions: Extensions {
                namespace: None,
                vary: None,
                stats: Some(create_stats()),
            },
        };
        
        // 更新索引
        {
            let mut index = self.inner.index.write().await;
            index.put(key.clone(), metadata);
        }
        
        Ok(())
    }
    
    /// 失效缓存
    pub async fn invalidate(&self, key: &str) -> Result<()> {
        // 获取元数据以删除文件
        let metadata = {
            let mut index = self.inner.index.write().await;
            index.pop(key)
        };
        
        if let Some(metadata) = metadata {
            // 删除文件（如果是文件存储）
            if let StorageLocation::File(file_path) = &metadata.storage.location {
                let _ = delete_file(&self.inner.disk_path, file_path).await;
            }
            
            // 从命名空间索引中移除
            if let Some(ref ns_info) = metadata.extensions.namespace {
                if let Some(mut keys) = self.inner.namespace_index.get_mut(&ns_info.namespace) {
                    keys.retain(|k| k != key);
                }
                
                // 从标签索引中移除
                for tag in &ns_info.tags {
                    if let Some(mut keys) = self.inner.tag_index.get_mut(tag) {
                        keys.retain(|k| k != key);
                    }
                }
            }
            
            // 从 Vary 索引中移除
            if metadata.extensions.vary.is_some() {
                self.inner.vary_index.remove(key);
            }
        }
        
        Ok(())
    }
    
    /// 检查键是否存在
    pub async fn exists(&self, key: &str) -> bool {
        let index = self.inner.index.read().await;
        index.peek(key).map(|m| !m.is_expired()).unwrap_or(false)
    }
    
    // ============ 高级操作 ============
    
    /// 设置缓存（带命名空间）
    pub async fn set_with_namespace(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
        namespace: NamespaceInfo,
    ) -> Result<()> {
        // 先设置缓存
        self.set(key.clone(), data, content_type).await?;
        
        // 更新元数据添加命名空间信息
        {
            let mut index = self.inner.index.write().await;
            if let Some(metadata) = index.get_mut(&key) {
                metadata.extensions.namespace = Some(namespace.clone());
            }
        }
        
        // 更新命名空间索引
        self.inner.namespace_index
            .entry(namespace.namespace.clone())
            .or_insert_with(Vec::new)
            .push(key.clone());
        
        // 更新标签索引
        for tag in &namespace.tags {
            self.inner.tag_index
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(key.clone());
        }
        
        Ok(())
    }
    
    /// 失效整个命名空间
    pub async fn invalidate_namespace(&self, namespace: &str) -> Result<usize> {
        let keys = self.inner.namespace_index
            .remove(namespace)
            .map(|(_, keys)| keys)
            .unwrap_or_default();
        
        let mut count = 0;
        for key in &keys {
            if self.invalidate(key).await.is_ok() {
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// 失效多个标签
    pub async fn invalidate_tags(&self, tags: &[String]) -> Result<usize> {
        let mut keys_to_invalidate = std::collections::HashSet::new();
        
        for tag in tags {
            if let Some((_, keys)) = self.inner.tag_index.remove(tag) {
                for key in keys {
                    keys_to_invalidate.insert(key);
                }
            }
        }
        
        let mut count = 0;
        for key in keys_to_invalidate {
            if self.invalidate(&key).await.is_ok() {
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// 获取元数据（不读取数据）
    pub async fn get_metadata(&self, key: &str) -> Option<CacheMetadata> {
        let mut index = self.inner.index.write().await;
        index.get(key).cloned()
    }
    
    /// 批量获取
    pub async fn get_batch(&self, keys: &[String]) -> Vec<Option<Vec<u8>>> {
        let mut results = Vec::with_capacity(keys.len());
        for key in keys {
            results.push(self.get(key).await.ok().flatten());
        }
        results
    }
    
    // ============ Vary 支持 ============
    
    /// 设置 Vary 缓存
    pub async fn set_with_vary(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
        vary_conditions: Vec<VaryCondition>,
    ) -> Result<()> {
        // 注意：这里需要提供 vary_values 才能生成完整的键
        // 为了简化，我们假设使用默认的 VaryValues
        let vary_values = VaryValues::default();
        let variant_key = generate_variant_key(&key, &vary_conditions, &vary_values);
        
        // 创建 VaryInfo
        let vary_info = VaryInfo {
            vary_on: vary_conditions.clone(),
            variant_key: variant_key.clone(),
        };
        
        let full_key = build_cache_key(&key, &vary_info);
        
        // 设置缓存
        self.set(full_key.clone(), data, content_type).await?;
        
        // 更新元数据添加 Vary 信息
        {
            let mut index = self.inner.index.write().await;
            if let Some(metadata) = index.get_mut(&full_key) {
                metadata.extensions.vary = Some(vary_info.clone());
            }
        }
        
        // 更新 Vary 索引
        self.inner.vary_index.insert(key, vary_info);
        
        Ok(())
    }
    
    /// 获取 Vary 缓存（需要提供变量值）
    pub async fn get_vary(
        &self,
        key: &str,
        vary_values: &VaryValues,
    ) -> Result<Option<Vec<u8>>> {
        // 获取 Vary 信息
        let vary_info = self.inner.vary_index.get(key)
            .map(|entry| entry.value().clone())
            .ok_or_else(|| CacheError::KeyNotFound(key.to_string()))?;
        
        // 生成变种键
        let variant_key = generate_variant_key(key, &vary_info.vary_on, vary_values);
        let full_key = build_cache_key(key, &VaryInfo {
            vary_on: vary_info.vary_on,
            variant_key,
        });
        
        self.get(&full_key).await
    }
    
    // ============ 统计和监控 ============
    
    /// 获取统计信息
    pub async fn statistics(&self) -> CacheStatistics {
        let index = self.inner.index.read().await;
        let total_entries = index.len();
        
        // 计算内存使用（粗略估算）
        let memory_mb = (total_entries * 512) as f64 / (1024.0 * 1024.0); // 假设每个条目平均 512 字节
        
        // 计算磁盘使用（需要遍历文件，这里简化处理）
        let disk_gb = 0.0; // TODO: 实现磁盘使用统计
        
        self.inner.stats.get_statistics(total_entries, memory_mb, disk_gb)
    }
    
    /// 获取热点键（Top N）
    pub async fn hot_keys(&self, top_n: usize) -> Vec<(String, u32)> {
        let index = self.inner.index.read().await;
        let mut entries: Vec<(String, u32)> = index.iter()
            .filter_map(|(key, metadata)| {
                metadata.extensions.stats.as_ref().map(|stats| {
                    (key.clone(), stats.access_count)
                })
            })
            .collect();
        
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(top_n);
        entries
    }
    
    // ============ 维护操作 ============
    
    /// 清理过期缓存
    pub async fn cleanup_expired(&self) -> Result<usize> {
        let expired_keys: Vec<String> = {
            let index = self.inner.index.read().await;
            index.iter()
                .filter(|(_, metadata)| metadata.is_expired())
                .map(|(key, _)| key.clone())
                .collect()
        };
        
        let mut count = 0;
        for key in expired_keys {
            if self.invalidate(&key).await.is_ok() {
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// 清空所有缓存
    pub async fn clear(&self) -> Result<()> {
        // 清空索引
        {
            let mut index = self.inner.index.write().await;
            index.clear();
        }
        
        // 清空命名空间索引
        self.inner.namespace_index.clear();
        
        // 清空标签索引
        self.inner.tag_index.clear();
        
        // 清空 Vary 索引
        self.inner.vary_index.clear();
        
        // 删除所有缓存文件
        if let Err(e) = tokio::fs::remove_dir_all(&self.inner.disk_path).await {
            tracing::warn!("Failed to remove cache directory: {}", e);
        }
        
        // 重新创建目录
        tokio::fs::create_dir_all(&self.inner.disk_path)
            .await
            .map_err(|e| CacheError::Io(e))?;
        
        Ok(())
    }
    
    /// 保存索引到磁盘
    pub async fn save_index(&self) -> Result<()> {
        let index_path = self.inner.disk_path.join("index.bin");
        let index = self.inner.index.read().await;
        
        // 序列化索引（收集为 Vec）
        let entries: Vec<(String, CacheMetadata)> = index.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        let serialized = bincode::serialize(&entries)
            .map_err(|e| CacheError::Serialization(format!("Failed to serialize index: {}", e)))?;
        
        tokio::fs::write(&index_path, serialized)
            .await
            .map_err(|e| CacheError::Io(e))?;
        
        Ok(())
    }
    
    /// 从磁盘加载索引
    pub async fn load_index(&self) -> Result<()> {
        let index_path = self.inner.disk_path.join("index.bin");
        
        if !index_path.exists() {
            return Ok(());
        }
        
        let data = tokio::fs::read(&index_path)
            .await
            .map_err(|e| CacheError::Io(e))?;
        
        let entries: Vec<(String, CacheMetadata)> = bincode::deserialize(&data)
            .map_err(|e| CacheError::Serialization(format!("Failed to deserialize index: {}", e)))?;
        
        let mut index = self.inner.index.write().await;
        for (key, metadata) in entries {
            index.put(key, metadata);
        }
        
        Ok(())
    }
}

