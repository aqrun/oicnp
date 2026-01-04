use crate::metadata::{CacheMetadata, FetchStatus};
use crate::error::{CacheError, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::Duration;

/// 回源保护管理器
pub struct FetchProtection {
    /// 正在获取的键集合
    fetching: Arc<RwLock<HashMap<String, Arc<tokio::sync::Notify>>>>,
    /// 超时时间
    timeout: Duration,
}

impl FetchProtection {
    pub fn new(timeout_seconds: u64) -> Self {
        Self {
            fetching: Arc::new(RwLock::new(HashMap::new())),
            timeout: Duration::from_secs(timeout_seconds),
        }
    }
    
    /// 尝试获取锁，如果已经有其他请求在获取，则等待
    pub async fn acquire_fetch_lock(&self, key: &str) -> Result<FetchGuard> {
        let notify_to_wait = {
            let fetching = self.fetching.read().await;
            fetching.get(key).cloned()
        };
        
        if let Some(notify) = notify_to_wait {
            // 已经有请求在获取，等待完成
            tokio::select! {
                _ = notify.notified() => {
                    // 等待完成，返回 None 表示应该从缓存读取
                    return Ok(FetchGuard::Waited);
                }
                _ = tokio::time::sleep(self.timeout) => {
                    return Err(CacheError::FetchTimeout(key.to_string()));
                }
            }
        }
        
        // 创建新的通知器
        let notify = Arc::new(tokio::sync::Notify::new());
        {
            let mut fetching = self.fetching.write().await;
            fetching.insert(key.to_string(), notify.clone());
        }
        
        Ok(FetchGuard::Acquired { key: key.to_string(), notify, fetching: self.fetching.clone() })
    }
    
    /// 检查是否正在获取
    pub async fn is_fetching(&self, key: &str) -> bool {
        let fetching = self.fetching.read().await;
        fetching.contains_key(key)
    }
}

/// 获取锁的守卫
pub enum FetchGuard {
    /// 已获取锁，可以执行获取操作
    Acquired {
        key: String,
        notify: Arc<tokio::sync::Notify>,
        fetching: Arc<RwLock<HashMap<String, Arc<tokio::sync::Notify>>>>,
    },
    /// 等待其他请求完成
    Waited,
}

impl FetchGuard {
    /// 释放锁并通知等待的请求
    pub async fn release(self) {
        if let FetchGuard::Acquired { key, notify, fetching } = self {
            let mut map = fetching.write().await;
            map.remove(&key);
            drop(map);
            notify.notify_waiters();
        }
    }
}

/// 更新元数据的获取状态
pub fn update_fetch_status(
    metadata: &mut CacheMetadata,
    status: FetchStatus,
) {
    metadata.fetch_status = status;
    metadata.last_fetch_attempt = chrono::Utc::now().timestamp();
}

