use serde::{Deserialize, Serialize};
use crate::metadata::CompressionAlgorithm;

#[derive(Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 索引容量（元数据条目数）
    pub index_capacity: usize,
    
    /// 默认 TTL（秒）
    /// - `> 0`: 正常过期时间
    /// - `= 0`: 永不过期
    /// - `< 0`: 立即过期（不推荐）
    pub default_ttl_seconds: i64,
    
    /// 磁盘缓存根目录
    pub disk_path: String,
    
    /// 存储配置
    pub storage: StorageConfig,
    
    /// 压缩配置
    pub compression: CompressionConfig,
    
    /// 并发配置
    pub concurrency: ConcurrencyConfig,
    
    /// SWR 配置（Stale-While-Revalidate）
    pub swr: SwrConfig,

    /// 服务端监听配置
    pub server: ServerConfig,
}

/// 独立服务监听地址配置（仅 oic-cache-server 使用）
#[derive(Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Redis 协议监听地址
    pub redis_addr: String,
    /// gRPC 监听地址
    pub grpc_addr: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 内联存储阈值（字节）
    pub inline_threshold: u64,
    
    /// 流式读取阈值（字节）
    pub streaming_threshold: u64,
    
    /// 是否启用磁盘持久化
    pub enable_persistence: bool,
    
    /// 是否在启动时自动加载索引
    pub auto_load_index: bool,
    
    /// 是否启用自动保存索引
    pub auto_save_index: bool,
    
    /// 自动保存索引的间隔（秒），定期保存的兜底机制
    pub auto_save_interval_seconds: u64,
    
    /// 更新后延迟保存的时间（毫秒），重置式 debounce
    /// 每次更新会重置计时器，只有在最后一次更新后等待此时间才保存
    pub auto_save_debounce_ms: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SwrConfig {
    /// 是否启用 Stale-While-Revalidate (SWR) 功能
    pub enabled: bool,
    
    /// Stale 数据的最大保留时间（秒）
    /// 超过此时间后，即使启用 SWR 也不会返回过期数据
    /// 0 表示不限制，只要数据存在就返回
    pub max_stale_seconds: i64,
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
            default_ttl_seconds: 360, // 默认缓存时间 10 分钟
            disk_path: "./target/.cache".to_string(),
            storage: StorageConfig {
                inline_threshold: 4096,
                streaming_threshold: 10 * 1024 * 1024,
                enable_persistence: true,
                auto_load_index: true,
                auto_save_index: true,
                auto_save_interval_seconds: 30, // 每 30 秒定期保存一次（兜底）
                auto_save_debounce_ms: 2000, // 更新后延迟 2 秒保存（重置式 debounce）
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
            swr: SwrConfig {
                enabled: false,
                max_stale_seconds: 3600, // 默认最多保留 1 小时的 stale 数据
            },
            server: ServerConfig {
                redis_addr: "0.0.0.0:6380".to_string(),
                grpc_addr: "0.0.0.0:50051".to_string(),
            },
        }
    }
}

