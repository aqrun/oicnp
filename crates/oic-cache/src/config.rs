use serde::{Deserialize, Serialize};
use crate::metadata::CompressionAlgorithm;

#[derive(Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 索引容量（元数据条目数）
    pub index_capacity: usize,
    
    /// 默认 TTL（秒）
    pub default_ttl_seconds: i64,
    
    /// 磁盘缓存根目录
    pub disk_path: String,
    
    /// 存储配置
    pub storage: StorageConfig,
    
    /// 压缩配置
    pub compression: CompressionConfig,
    
    /// 并发配置
    pub concurrency: ConcurrencyConfig,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 内联存储阈值（字节）
    pub inline_threshold: u64,
    
    /// 流式读取阈值（字节）
    pub streaming_threshold: u64,
    
    /// 是否启用磁盘持久化
    pub enable_persistence: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// 是否启用压缩
    pub enabled: bool,
    
    /// 默认压缩算法
    pub default_algorithm: CompressionAlgorithm,
    
    /// 压缩阈值（小于此大小不压缩）
    pub min_size: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// 最大并发写入数
    pub max_concurrent_writes: usize,
    
    /// 回源超时时间（秒）
    pub fetch_timeout_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            index_capacity: 10_000,
            default_ttl_seconds: 3600,
            disk_path: "./cache".to_string(),
            storage: StorageConfig {
                inline_threshold: 4096,
                streaming_threshold: 10 * 1024 * 1024,
                enable_persistence: true,
            },
            compression: CompressionConfig {
                enabled: true,
                default_algorithm: CompressionAlgorithm::Zstd,
                min_size: 1024,
            },
            concurrency: ConcurrencyConfig {
                max_concurrent_writes: 1000,
                fetch_timeout_seconds: 30,
            },
        }
    }
}

