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
use std::sync::atomic::AtomicBool;
use tokio::sync::RwLock;
use tokio::sync::Notify;
use bytes::Bytes;

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
    
    /// 自动保存通知器
    save_notify: Arc<Notify>,
    
    /// 是否正在运行自动保存任务
    auto_save_running: Arc<AtomicBool>,
}

impl Cache {
    /// 创建新的缓存实例
    /// 
    /// 注意：如果配置了 `auto_load_index`，请使用 `new_with_auto_load` 或 `from_config_file`
    /// 来异步加载索引
    pub fn new(config: CacheConfig) -> Self {
        let capacity = NonZeroUsize::new(config.index_capacity).unwrap_or(NonZeroUsize::new(1000).unwrap());
        let disk_path = PathBuf::from(&config.disk_path);
        
        // 确保目录存在
        if let Err(e) = std::fs::create_dir_all(&disk_path) {
            tracing::warn!("Failed to create cache directory: {}", e);
        }
        
        let inner = Arc::new(CacheInner {
            index: Arc::new(RwLock::new(LruCache::new(capacity))),
            namespace_index: Arc::new(DashMap::new()),
            tag_index: Arc::new(DashMap::new()),
            vary_index: Arc::new(DashMap::new()),
            config: config.clone(),
            disk_path,
            stats: Statistics::new(),
            fetch_protection: FetchProtection::new(config.concurrency.fetch_timeout_seconds),
            save_notify: Arc::new(Notify::new()),
            auto_save_running: Arc::new(AtomicBool::new(false)),
        });
        
        let cache = Self { inner: inner.clone() };
        
        // 如果启用了自动保存，启动后台任务
        if config.storage.auto_save_index {
            cache.start_auto_save_task();
        }
        
        cache
    }
    
    /// 从配置文件加载
    pub async fn from_config_file(path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| CacheError::Io(e))?;
        let config: CacheConfig = toml::from_str(&content)
            .map_err(|e| CacheError::InvalidConfig(format!("Failed to parse config: {}", e)))?;
        let cache = Self::new(config);
        // 自动加载索引
        cache.load_index().await?;
        Ok(cache)
    }
    
    /// 创建缓存实例并自动加载索引（如果存在）
    pub async fn new_with_auto_load(config: CacheConfig) -> Result<Self> {
        let cache = Self::new(config);
        // 自动加载索引
        cache.load_index().await?;
        Ok(cache)
    }
    
    /// 启动自动保存任务（使用重置式 debounce）
    fn start_auto_save_task(&self) {
        let inner = self.inner.clone();
        let save_notify = self.inner.save_notify.clone();
        let auto_save_running = self.inner.auto_save_running.clone();
        let interval_seconds = self.inner.config.storage.auto_save_interval_seconds;
        let debounce_ms = self.inner.config.storage.auto_save_debounce_ms;
        
        // 检查是否已经在运行
        if auto_save_running.compare_exchange(false, true, std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed).is_err() {
            return; // 已经在运行
        }
        
        tokio::spawn(async move {
            use tokio::time::{Instant, Duration};
            
            // 定期保存计时器（如果启用）
            let mut interval_timer = if interval_seconds > 0 {
                Some(tokio::time::interval(Duration::from_secs(interval_seconds)))
            } else {
                None
            };
            
            // Debounce 相关状态
            let mut last_update_time: Option<Instant> = None;
            let debounce_duration = Duration::from_millis(debounce_ms);
            
            loop {
                // 检查是否需要立即保存（debounce 已超时）
                if let Some(update_time) = last_update_time {
                    if update_time.elapsed() >= debounce_duration {
                        // 已经超时，立即保存
                        if let Err(e) = Self::save_index_inner(&inner).await {
                            tracing::warn!("Failed to auto-save index (debounced): {}", e);
                        }
                        last_update_time = None;
                        continue; // 重新开始循环
                    }
                }
                
                // 计算下次 debounce 等待时间
                let debounce_wait = last_update_time.and_then(|update_time| {
                    let elapsed = update_time.elapsed();
                    if elapsed < debounce_duration {
                        Some(debounce_duration - elapsed)
                    } else {
                        None // 已经超时，会在下次循环处理
                    }
                });
                
                tokio::select! {
                    // 定期保存（兜底机制）
                    _ = interval_timer.as_mut().unwrap().tick(), if interval_timer.is_some() => {
                        // 定期保存触发
                        if let Err(e) = Self::save_index_inner(&inner).await {
                            tracing::warn!("Failed to auto-save index (periodic): {}", e);
                        }
                        // 重置 debounce 状态（避免重复保存）
                        last_update_time = None;
                    }
                    
                    // Debounce 计时器到期（重置式 debounce）
                    _ = tokio::time::sleep(debounce_wait.unwrap_or(Duration::from_secs(3600))), if debounce_wait.is_some() => {
                        // 再次检查是否仍然需要保存（可能在等待期间又有更新）
                        if let Some(update_time) = last_update_time {
                            if update_time.elapsed() >= debounce_duration {
                                // Debounce 时间到，执行保存
                                if let Err(e) = Self::save_index_inner(&inner).await {
                                    tracing::warn!("Failed to auto-save index (debounced): {}", e);
                                }
                                last_update_time = None;
                            }
                        }
                    }
                    
                    // 收到更新通知
                    _ = save_notify.notified() => {
                        // 重置 debounce 计时器（重置式 debounce）
                        last_update_time = Some(Instant::now());
                    }
                }
            }
        });
    }
    
    /// 触发自动保存（异步，不阻塞）
    fn trigger_auto_save(&self) {
        if self.inner.config.storage.auto_save_index {
            self.inner.save_notify.notify_one();
        }
    }
    
    /// 内部保存索引实现
    async fn save_index_inner(inner: &CacheInner) -> Result<()> {
        let index_path = inner.disk_path.join("index.bin");
        let index = inner.index.read().await;
        
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
    
    // ============ 基础操作 ============
    
    /// 获取缓存（原始字节）
    /// 
    /// 返回 `Bytes` 类型，Clone 是零拷贝的（引用计数）
    pub async fn get(&self, key: &str) -> Result<Option<Bytes>> {
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
        let is_expired = metadata.is_expired();
        let is_stale_acceptable = if is_expired && self.inner.config.swr.enabled {
            metadata.is_stale_acceptable(self.inner.config.swr.max_stale_seconds)
        } else {
            false
        };
        
        if is_expired && !is_stale_acceptable {
            // 过期且不能作为 stale 返回，删除并返回 None
            self.inner.stats.record_miss();
            // 异步删除过期项
            let _ = self.invalidate(key).await;
            return Ok(None);
        }
        
        // 检查是否有效（未过期且状态为成功）
        let is_valid = metadata.is_valid();
        
        if !is_valid && !is_stale_acceptable {
            // 无效且不能作为 stale 返回
            self.inner.stats.record_miss();
            return Ok(None);
        }
        
        // 如果使用 stale 数据，标记需要重新获取
        if is_stale_acceptable && !is_valid {
            // 标记为需要重新获取（但不阻塞返回）
            let inner_clone = self.inner.clone();
            let key_clone = key.to_string();
            tokio::spawn(async move {
                // 更新状态为 Fetching，表示正在重新获取
                let mut index = inner_clone.index.write().await;
                if let Some(metadata) = index.get_mut(&key_clone) {
                    if metadata.fetch_status != FetchStatus::Fetching {
                        metadata.fetch_status = FetchStatus::Fetching;
                        metadata.last_fetch_attempt = chrono::Utc::now().timestamp();
                    }
                }
            });
        }
        
        // 读取数据
        let data = match &metadata.storage.location {
            StorageLocation::Inline(_) => {
                read_inline(&metadata.storage.location)
                    .ok_or_else(|| CacheError::KeyNotFound(key.to_string()))?
            }
            StorageLocation::File(_) => {
                let file_data = read_file(&metadata.storage.location, &self.inner.disk_path)
                    .await?
                    .ok_or_else(|| CacheError::KeyNotFound(key.to_string()))?;
                
                // 解压数据
                if let Some(ref compression) = metadata.storage.compression {
                    decompress(file_data.as_ref(), compression)?
                } else {
                    file_data
                }
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
        data: impl Into<Bytes>,
        content_type: String,
    ) -> Result<()> {
        self.set_with_ttl(key, data, content_type, self.inner.config.default_ttl_seconds).await
    }
    
    /// 设置缓存（指定 TTL）
    /// 
    /// # TTL 说明
    /// - `ttl_seconds > 0`: 正常过期时间
    /// - `ttl_seconds = 0`: 永不过期（使用 i64::MAX）
    /// - `ttl_seconds < 0`: 立即过期（使用 now - 1）
    pub async fn set_with_ttl(
        &self,
        key: String,
        data: impl Into<Bytes>,
        content_type: String,
        ttl_seconds: i64,
    ) -> Result<()> {
        let data = data.into();
        let now = chrono::Utc::now().timestamp();
        let size = data.len() as u64;
        
        // 决定存储策略
        let (storage_location, compression_info) = if size < self.inner.config.storage.inline_threshold {
            // 内联存储 - 零拷贝 Clone
            (StorageLocation::Inline(data.clone()), None)
        } else {
            // 文件存储
            let (final_data, compression) = if self.inner.config.compression.enabled
                && size >= self.inner.config.compression.min_size
            {
                let (compressed, comp_info) = compress(data.as_ref(), self.inner.config.compression.default_algorithm)?;
                (compressed, Some(comp_info))
            } else {
                (data.clone(), None)
            };
            
            let file_path = write_file(&self.inner.disk_path, &key, final_data.as_ref()).await?;
            (StorageLocation::File(file_path), compression)
        };
        
        // 计算过期时间
        // ttl_seconds = 0 表示永不过期，使用 i64::MAX
        // ttl_seconds < 0 表示立即过期，使用 now - 1
        let expires_at = if ttl_seconds == 0 {
            i64::MAX
        } else if ttl_seconds < 0 {
            now - 1
        } else {
            now + ttl_seconds
        };
        
        // 创建元数据
        let metadata = CacheMetadata {
            version: 1,
            key: key.clone(),
            size,
            created_at: now,
            expires_at,
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
        
        // 触发自动保存
        self.trigger_auto_save();
        
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
            
            // 触发自动保存
            self.trigger_auto_save();
        }
        
        Ok(())
    }
    
    /// 检查键是否存在（包括 stale 数据，如果启用了 SWR）
    pub async fn exists(&self, key: &str) -> bool {
        let metadata = {
            let index = self.inner.index.read().await;
            index.peek(key).cloned()
        };
        
        match metadata {
            Some(m) => {
                // 如果启用 SWR，stale 数据也算存在
                if self.inner.config.swr.enabled {
                    m.is_valid() || m.is_stale_acceptable(self.inner.config.swr.max_stale_seconds)
                } else {
                    m.is_valid()
                }
            }
            None => false,
        }
    }
    
    /// 检查数据是否是 stale（过期但可接受）
    pub async fn is_stale(&self, key: &str) -> bool {
        if !self.inner.config.swr.enabled {
            return false;
        }
        
        let metadata = {
            let index = self.inner.index.read().await;
            index.peek(key).cloned()
        };
        
        match metadata {
            Some(m) => m.is_expired() && m.is_stale_acceptable(self.inner.config.swr.max_stale_seconds),
            None => false,
        }
    }
    
    // ============ 高级操作 ============
    
    /// 设置缓存（带命名空间）
    pub async fn set_with_namespace(
        &self,
        key: String,
        data: impl Into<Bytes>,
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
        
        // 触发自动保存
        self.trigger_auto_save();
        
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
    pub async fn get_batch(&self, keys: &[String]) -> Vec<Option<Bytes>> {
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
        data: impl Into<Bytes>,
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
        
        // 触发自动保存
        self.trigger_auto_save();
        
        Ok(())
    }
    
    /// 获取 Vary 缓存（需要提供变量值）
    pub async fn get_vary(
        &self,
        key: &str,
        vary_values: &VaryValues,
    ) -> Result<Option<Bytes>> {
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
    /// 
    /// 清理策略：
    /// - 如果启用了 SWR：只清理过期且超过 max_stale_seconds 的数据
    /// - 如果未启用 SWR：清理所有过期的数据
    pub async fn cleanup_expired(&self) -> Result<usize> {
        let expired_keys: Vec<String> = {
            let index = self.inner.index.read().await;
            index.iter()
                .filter(|(_, metadata)| {
                    if !metadata.is_expired() {
                        return false; // 未过期，不清理
                    }
                    
                    // 如果启用了 SWR，需要检查是否超过 max_stale 时间
                    if self.inner.config.swr.enabled {
                        // 如果 max_stale_seconds = 0，表示不限制 stale 时间，不清理
                        if self.inner.config.swr.max_stale_seconds == 0 {
                            return false;
                        }
                        
                        // 检查是否超过 max_stale 时间
                        let now = chrono::Utc::now().timestamp();
                        let stale_age = now - metadata.expires_at;
                        stale_age > self.inner.config.swr.max_stale_seconds
                    } else {
                        // 未启用 SWR，清理所有过期数据
                        true
                    }
                })
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
        
        // 触发自动保存（清空后保存空索引）
        self.trigger_auto_save();
        
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
    
    /// 从磁盘加载索引并重建所有辅助索引
    pub async fn load_index(&self) -> Result<()> {
        let index_path = self.inner.disk_path.join("index.bin");
        
        if !index_path.exists() {
            tracing::info!("No index file found, starting with empty cache");
            return Ok(());
        }
        
        let data = tokio::fs::read(&index_path)
            .await
            .map_err(|e| CacheError::Io(e))?;
        
        let entries: Vec<(String, CacheMetadata)> = bincode::deserialize(&data)
            .map_err(|e| CacheError::Serialization(format!("Failed to deserialize index: {}", e)))?;
        
        let mut index = self.inner.index.write().await;
        let mut namespace_keys: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        let mut tag_keys: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        let mut vary_map: std::collections::HashMap<String, VaryInfo> = std::collections::HashMap::new();
        
        let mut loaded_count = 0;
        let mut expired_count = 0;
        
        for (key, metadata) in entries {
            // 检查是否过期且需要清理
            // 如果启用了 SWR，只清理超过 max_stale_seconds 的数据
            let should_skip = if !metadata.is_expired() {
                false // 未过期，不跳过
            } else if self.inner.config.swr.enabled {
                // 如果启用了 SWR，检查是否超过 max_stale 时间
                if self.inner.config.swr.max_stale_seconds == 0 {
                    false // max_stale_seconds = 0 表示不限制，保留数据
                } else {
                    let now = chrono::Utc::now().timestamp();
                    let stale_age = now - metadata.expires_at;
                    stale_age > self.inner.config.swr.max_stale_seconds
                }
            } else {
                true // 未启用 SWR，删除所有过期数据
            };
            
            if should_skip {
                expired_count += 1;
                // 如果文件存在，删除文件
                if let StorageLocation::File(file_path) = &metadata.storage.location {
                    let _ = delete_file(&self.inner.disk_path, file_path).await;
                }
                continue;
            }
            
            // 重建命名空间索引
            if let Some(ref ns_info) = metadata.extensions.namespace {
                namespace_keys
                    .entry(ns_info.namespace.clone())
                    .or_insert_with(Vec::new)
                    .push(key.clone());
                
                // 重建标签索引
                for tag in &ns_info.tags {
                    tag_keys
                        .entry(tag.clone())
                        .or_insert_with(Vec::new)
                        .push(key.clone());
                }
            }
            
            // 重建 Vary 索引
            if let Some(ref vary_info) = metadata.extensions.vary {
                // 从完整键中提取基础键（移除 variant_key 部分）
                // 这里简化处理，假设基础键就是 key 本身
                vary_map.insert(key.clone(), vary_info.clone());
            }
            
            index.put(key, metadata);
            loaded_count += 1;
        }
        
        // 将重建的索引写入 DashMap
        for (namespace, keys) in namespace_keys {
            self.inner.namespace_index.insert(namespace, keys);
        }
        
        for (tag, keys) in tag_keys {
            self.inner.tag_index.insert(tag, keys);
        }
        
        for (key, vary_info) in vary_map {
            self.inner.vary_index.insert(key, vary_info);
        }
        
        tracing::info!(
            "Loaded {} cache entries from index ({} expired entries skipped)",
            loaded_count,
            expired_count
        );
        
        Ok(())
    }
}

