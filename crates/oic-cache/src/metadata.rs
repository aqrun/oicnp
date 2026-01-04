use serde::{Deserialize, Serialize};

// ============ 第一层：核心字段 ============

#[derive(Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    /// 版本号
    pub version: u8,
    
    /// 缓存键
    pub key: String,
    
    /// 数据大小（字节）
    pub size: u64,
    
    /// 创建时间戳
    pub created_at: i64,
    
    /// 过期时间戳
    pub expires_at: i64,
    
    /// ⭐ 回源保护
    pub fetch_status: FetchStatus,
    pub last_fetch_attempt: i64,
    
    /// 存储信息
    pub storage: StorageInfo,
    
    /// 内容信息
    pub content: ContentInfo,
    
    /// 扩展字段
    pub extensions: Extensions,
}

// ⭐ 补充：回源状态枚举
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum FetchStatus {
    Success,
    Fetching,
    Failed,
}

// ============ 第二层：存储信息 ============

#[derive(Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    /// 存储位置（包含策略信息）
    pub location: StorageLocation,
    
    /// 压缩信息
    pub compression: Option<CompressionInfo>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum StorageLocation {
    /// 内联存储（< 4KB）
    Inline(Vec<u8>),
    
    /// 文件存储（>= 4KB）
    File(String),
}

// ⭐ 优化：压缩信息使用 enum
#[derive(Clone, Serialize, Deserialize)]
pub enum CompressionInfo {
    /// 未压缩
    None,
    
    /// 已压缩
    Compressed {
        original_size: u64,
        compressed_size: u64,
        algorithm: CompressionAlgorithm,
    },
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum CompressionAlgorithm {
    Gzip,
    Zstd,
    Brotli,
}

// ⭐ 辅助方法
impl StorageInfo {
    /// 根据存储位置判断策略
    pub fn strategy(&self) -> StorageStrategy {
        match &self.location {
            StorageLocation::Inline(_) => StorageStrategy::InlineMemory,
            StorageLocation::File(_) => {
                // 根据大小判断
                let size = self.compression
                    .as_ref()
                    .and_then(|c| match c {
                        CompressionInfo::Compressed { original_size, .. } => Some(*original_size),
                        _ => None,
                    })
                    .unwrap_or(0);
                
                if size > 10 * 1024 * 1024 {
                    StorageStrategy::Streaming
                } else {
                    StorageStrategy::DiskBacked
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StorageStrategy {
    InlineMemory,
    DiskBacked,
    Streaming,
}

// ============ 第三层：内容信息 ============

#[derive(Clone, Serialize, Deserialize)]
pub struct ContentInfo {
    pub content_type: String,
    pub etag: String,
    pub encoding: Option<String>,
    pub charset: Option<String>,
}

// ============ 第四层：扩展字段 ============

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Extensions {
    pub namespace: Option<NamespaceInfo>,
    pub vary: Option<VaryInfo>,
    pub stats: Option<StatsInfo>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NamespaceInfo {
    pub namespace: String,
    pub tags: Vec<String>,
    pub priority: CachePriority,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum CachePriority {
    Low = 0,
    Normal = 1,
    High = 2,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VaryInfo {
    pub vary_on: Vec<VaryCondition>,
    pub variant_key: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum VaryCondition {
    AcceptLanguage,
    AcceptEncoding,
    UserAgent,
    Custom(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StatsInfo {
    pub access_count: u32,
    pub last_accessed: i64,
    pub access_frequency: f64,
    pub hit_count: u32,
    pub avg_read_time_us: u64,
    pub total_bytes_served: u64,
}

impl CacheMetadata {
    /// 检查是否过期
    /// 
    /// 注意：如果 `expires_at` 为 `i64::MAX`，表示永不过期，返回 `false`
    pub fn is_expired(&self) -> bool {
        // i64::MAX 表示永不过期
        if self.expires_at == i64::MAX {
            return false;
        }
        
        let now = chrono::Utc::now().timestamp();
        now >= self.expires_at
    }
    
    /// 检查是否有效（未过期且状态为成功）
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && self.fetch_status == FetchStatus::Success
    }
    
    /// 检查是否可以作为 stale 数据返回（过期但未超过 max_stale 时间）
    pub fn is_stale_acceptable(&self, max_stale_seconds: i64) -> bool {
        if !self.is_expired() {
            return false; // 未过期，不是 stale
        }
        
        if max_stale_seconds == 0 {
            return true; // 不限制 stale 时间
        }
        
        let now = chrono::Utc::now().timestamp();
        let stale_age = now - self.expires_at;
        stale_age <= max_stale_seconds
    }
}

