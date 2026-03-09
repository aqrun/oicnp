use serde::{Deserialize, Serialize};
use crate::metadata::CompressionAlgorithm;

#[derive(Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 索引容量（元数据条目数）
    #[serde(default = "default_index_capacity")]
    pub index_capacity: usize,
    
    /// 默认 TTL（秒）
    /// - `> 0`: 正常过期时间
    /// - `= 0`: 永不过期
    /// - `< 0`: 立即过期（不推荐）
    #[serde(default = "default_default_ttl_seconds")]
    pub default_ttl_seconds: i64,
    
    /// 磁盘缓存根目录
    #[serde(default = "default_disk_path")]
    pub disk_path: String,
    
    /// 存储配置
    #[serde(default)]
    pub storage: StorageConfig,
    
    /// 压缩配置
    #[serde(default)]
    pub compression: CompressionConfig,
    
    /// 并发配置
    #[serde(default)]
    pub concurrency: ConcurrencyConfig,
    
    /// SWR 配置（Stale-While-Revalidate）
    #[serde(default)]
    pub swr: SwrConfig,

    /// 服务端监听配置
    #[serde(default)]
    pub server: ServerConfig,
}

/// 独立服务监听地址配置（仅 oic-cache-server 使用）
#[derive(Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Redis 协议监听地址
    #[serde(default = "default_redis_addr")]
    pub redis_addr: String,
    /// gRPC 监听地址
    #[serde(default = "default_grpc_addr")]
    pub grpc_addr: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// 内联存储阈值（字节）
    #[serde(default = "default_inline_threshold")]
    pub inline_threshold: u64,
    
    /// 流式读取阈值（字节）
    #[serde(default = "default_streaming_threshold")]
    pub streaming_threshold: u64,
    
    /// 是否启用磁盘持久化
    #[serde(default = "default_enable_persistence")]
    pub enable_persistence: bool,
    
    /// 是否在启动时自动加载索引
    #[serde(default = "default_auto_load_index")]
    pub auto_load_index: bool,
    
    /// 是否启用自动保存索引
    #[serde(default = "default_auto_save_index")]
    pub auto_save_index: bool,
    
    /// 自动保存索引的间隔（秒），定期保存的兜底机制
    #[serde(default = "default_auto_save_interval_seconds")]
    pub auto_save_interval_seconds: u64,
    
    /// 更新后延迟保存的时间（毫秒），重置式 debounce
    /// 每次更新会重置计时器，只有在最后一次更新后等待此时间才保存
    #[serde(default = "default_auto_save_debounce_ms")]
    pub auto_save_debounce_ms: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SwrConfig {
    /// 是否启用 Stale-While-Revalidate (SWR) 功能
    #[serde(default = "default_swr_enabled")]
    pub enabled: bool,
    
    /// Stale 数据的最大保留时间（秒）
    /// 超过此时间后，即使启用 SWR 也不会返回过期数据
    /// 0 表示不限制，只要数据存在就返回
    #[serde(default = "default_swr_max_stale_seconds")]
    pub max_stale_seconds: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// 是否启用压缩
    #[serde(default = "default_compression_enabled")]
    pub enabled: bool,
    
    /// 默认压缩算法
    #[serde(default = "default_compression_algorithm")]
    pub default_algorithm: CompressionAlgorithm,
    
    /// 压缩阈值（小于此大小不压缩）
    #[serde(default = "default_compression_min_size")]
    pub min_size: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// 最大并发写入数
    #[serde(default = "default_max_concurrent_writes")]
    pub max_concurrent_writes: usize,
    
    /// 回源超时时间（秒）
    #[serde(default = "default_fetch_timeout_seconds")]
    pub fetch_timeout_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            index_capacity: default_index_capacity(),
            default_ttl_seconds: default_default_ttl_seconds(),
            disk_path: default_disk_path(),
            storage: StorageConfig::default(),
            compression: CompressionConfig::default(),
            concurrency: ConcurrencyConfig::default(),
            swr: SwrConfig::default(),
            server: ServerConfig::default(),
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            inline_threshold: default_inline_threshold(),
            streaming_threshold: default_streaming_threshold(),
            enable_persistence: default_enable_persistence(),
            auto_load_index: default_auto_load_index(),
            auto_save_index: default_auto_save_index(),
            auto_save_interval_seconds: default_auto_save_interval_seconds(),
            auto_save_debounce_ms: default_auto_save_debounce_ms(),
        }
    }
}

impl Default for SwrConfig {
    fn default() -> Self {
        Self {
            enabled: default_swr_enabled(),
            max_stale_seconds: default_swr_max_stale_seconds(),
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: default_compression_enabled(),
            default_algorithm: default_compression_algorithm(),
            min_size: default_compression_min_size(),
        }
    }
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_writes: default_max_concurrent_writes(),
            fetch_timeout_seconds: default_fetch_timeout_seconds(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            redis_addr: default_redis_addr(),
            grpc_addr: default_grpc_addr(),
        }
    }
}

fn default_index_capacity() -> usize {
    10_000
}

fn default_default_ttl_seconds() -> i64 {
    600 // 默认缓存时间 10 分钟
}

fn default_disk_path() -> String {
    "./target/.cache".to_string()
}

fn default_inline_threshold() -> u64 {
    4096
}

fn default_streaming_threshold() -> u64 {
    10 * 1024 * 1024
}

fn default_enable_persistence() -> bool {
    true
}

fn default_auto_load_index() -> bool {
    true
}

fn default_auto_save_index() -> bool {
    true
}

fn default_auto_save_interval_seconds() -> u64 {
    30 // 每 30 秒定期保存一次（兜底）
}

fn default_auto_save_debounce_ms() -> u64 {
    2000 // 更新后延迟 2 秒保存（重置式 debounce）
}

fn default_swr_enabled() -> bool {
    false
}

fn default_swr_max_stale_seconds() -> i64 {
    3600 // 默认最多保留 1 小时的 stale 数据
}

fn default_compression_enabled() -> bool {
    true
}

fn default_compression_algorithm() -> CompressionAlgorithm {
    CompressionAlgorithm::Zstd
}

fn default_compression_min_size() -> u64 {
    1024
}

fn default_max_concurrent_writes() -> usize {
    1000
}

fn default_fetch_timeout_seconds() -> u64 {
    30
}

fn default_redis_addr() -> String {
    "0.0.0.0:6380".to_string()
}

fn default_grpc_addr() -> String {
    "0.0.0.0:50051".to_string()
}

