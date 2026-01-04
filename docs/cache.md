# Rust 实现轻量级索引加文件存储缓存架构

轻量级索引 + 文件存储架构 内存只存索引元数据

暂时需要支持的功能：

* 数据大小阈值（4kb）
* 命名空间/分组
* Vary 变种缓存
* 回源保护
* 统计和热度追踪
* 并发写支持
* 序列化支持
* 无冗余字段
* 内存占用优化


```rust
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
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Clone, Copy, Serialize, Deserialize)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Clone, Serialize, Deserialize)]
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
```

## 1. 项目结构规划

```
cache_system/
├── Cargo.toml                    # 依赖配置
├── README.md                     # 项目说明
├── docs/
│   ├── architecture.md           # 架构设计文档（你当前的内容）
│   ├── api.md                    # API 使用文档
│   └── performance.md            # 性能指标和测试
├── src/
│   ├── lib.rs                    # 库入口
│   ├── metadata.rs               # 元数据结构（你的代码）
│   ├── cache.rs                  # 核心缓存实现
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── inline.rs             # 内联存储
│   │   ├── file.rs               # 文件存储
│   │   └── compression.rs        # 压缩处理
│   ├── vary.rs                   # Vary 处理
│   ├── stats.rs                  # 统计功能
│   ├── fetch.rs                  # 回源保护
│   ├── config.rs                 # 配置管理
│   ├── error.rs                  # 错误类型
│   └── utils.rs                  # 工具函数
├── examples/
│   ├── basic.rs                  # 基础使用示例
│   ├── axum_integration.rs       # Axum 集成示例
│   └── performance_test.rs       # 性能测试
├── tests/
│   ├── integration_test.rs       # 集成测试
│   └── concurrent_test.rs        # 并发测试
└── benches/
    └── benchmark.rs              # 性能基准测试

```

## 2. Cargo.toml 依赖清单

```toml
[package]
name = "oic-cache"
version = "0.1.0"
edition = "2024"

[dependencies]
# 异步运行时
tokio = { version = "1", features = ["full"] }

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"
bincode = "1"

# LRU 缓存
lru = "0.12"

# 并发数据结构
dashmap = "5"

# 压缩算法
flate2 = "1"          # Gzip
zstd = "0.13"         # Zstd（推荐）
brotli = "3"          # Brotli（可选）

# 哈希和加密
sha2 = "0.10"

# 时间处理
chrono = "0.4"

# 错误处理
thiserror = "1"
anyhow = "1"

# 日志
tracing = "0.1"
tracing-subscriber = "0.3"

# 性能监控（可选）
prometheus = { version = "0.13", optional = true }

[dev-dependencies]
criterion = "0.5"
tempfile = "3"

[features]
default = []
metrics = ["prometheus"]
```

## 3. 核心 API 接口规范

```rust
// src/cache.rs

use crate::metadata::*;
use std::sync::Arc;

/// 缓存系统的公开 API
pub struct Cache {
    inner: Arc<CacheInner>,
}

impl Cache {
    /// 创建新的缓存实例
    pub fn new(config: CacheConfig) -> Self;
    
    /// 从配置文件加载
    pub async fn from_config_file(path: &str) -> Result<Self>;
    
    // ============ 基础操作 ============
    
    /// 获取缓存
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    
    /// 设置缓存（自动判断存储策略）
    pub async fn set(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
    ) -> Result<()>;
    
    /// 设置缓存（指定 TTL）
    pub async fn set_with_ttl(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
        ttl_seconds: i64,
    ) -> Result<()>;
    
    /// 失效缓存
    pub async fn invalidate(&self, key: &str) -> Result<()>;
    
    /// 检查键是否存在
    pub async fn exists(&self, key: &str) -> bool;
    
    // ============ 高级操作 ============
    
    /// 设置缓存（带命名空间）
    pub async fn set_with_namespace(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
        namespace: NamespaceInfo,
    ) -> Result<()>;
    
    /// 失效整个命名空间
    pub async fn invalidate_namespace(&self, namespace: &str) -> Result<usize>;
    
    /// 失效多个标签
    pub async fn invalidate_tags(&self, tags: &[String]) -> Result<usize>;
    
    /// 获取元数据（不读取数据）
    pub async fn get_metadata(&self, key: &str) -> Option<CacheMetadata>;
    
    /// 批量获取
    pub async fn get_batch(&self, keys: &[String]) -> Vec<Option<Vec<u8>>>;
    
    // ============ Vary 支持 ============
    
    /// 设置 Vary 缓存
    pub async fn set_with_vary(
        &self,
        key: String,
        data: Vec<u8>,
        content_type: String,
        vary_conditions: Vec<VaryCondition>,
    ) -> Result<()>;
    
    /// 获取 Vary 缓存（需要提供变量值）
    pub async fn get_vary(
        &self,
        key: &str,
        vary_values: &VaryValues,
    ) -> Result<Option<Vec<u8>>>;
    
    // ============ 统计和监控 ============
    
    /// 获取统计信息
    pub async fn statistics(&self) -> CacheStatistics;
    
    /// 获取热点键（Top N）
    pub async fn hot_keys(&self, top_n: usize) -> Vec<(String, u32)>;
    
    // ============ 维护操作 ============
    
    /// 清理过期缓存
    pub async fn cleanup_expired(&self) -> Result<usize>;
    
    /// 清空所有缓存
    pub async fn clear(&self) -> Result<()>;
    
    /// 保存索引到磁盘
    pub async fn save_index(&self) -> Result<()>;
    
    /// 从磁盘加载索引
    pub async fn load_index(&self) -> Result<()>;
}

// ============ 辅助类型 ============

pub struct VaryValues {
    pub language: Option<String>,
    pub encoding: Option<String>,
    pub user_agent: Option<String>,
    pub custom: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CacheStatistics {
    pub total_requests: u64,
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub total_entries: usize,
    pub memory_usage_mb: f64,
    pub disk_usage_gb: f64,
}
```

## 4. 配置结构

```rust
// src/config.rs

use serde::{Deserialize, Serialize};

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
```

## 5. 错误类型定义

```rust
// src/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    #[error("Cache entry expired: {0}")]
    Expired(String),
    
    #[error("Compression error: {0}")]
    Compression(String),
    
    #[error("Concurrent write conflict: {0}")]
    WriteConflict(String),
    
    #[error("Fetch timeout: {0}")]
    FetchTimeout(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

pub type Result<T> = std::result::Result<T, CacheError>;
```

## 三、实现优先级和里程碑

阶段 1：核心功能（Week 1-2）

```
Milestone 1.0 - 基础缓存
├── ✅ 元数据结构（你已完成）
├── ☐ 基础 Cache 实现
│   ├── get/set 基本操作
│   ├── 内联存储（< 4KB）
│   └── 文件存储（>= 4KB）
├── ☐ LRU 淘汰策略
├── ☐ 过期时间检查
└── ☐ 基础测试

验收标准：
- 能存取数据
- 过期自动失效
- 内存使用符合预期
```

阶段 2：高级功能（Week 3-4）

```
Milestone 2.0 - 高级特性
├── ☐ 命名空间/分组
├── ☐ 压缩支持（Zstd）
├── ☐ 回源保护（防击穿）
├── ☐ 并发写保护
└── ☐ 统计功能

验收标准：
- 支持按命名空间失效
- 大数据自动压缩
- 并发测试通过
```

阶段 3：优化和扩展（Week 5-6）

```
Milestone 3.0 - 生产就绪
├── ☐ Vary 缓存
├── ☐ 索引持久化
├── ☐ 性能优化
│   ├── 批量操作
│   ├── 分片索引
│   └── 零拷贝读取
├── ☐ 监控集成
└── ☐ 文档完善

验收标准：
- 性能达标（见下方指标）
- 生产环境可用
- 文档齐全
```

## 5.1 单元测试

```rust
// tests/unit_test.rs

#[tokio::test]
async fn test_basic_get_set() {
    // 测试基础读写
}

#[tokio::test]
async fn test_expiration() {
    // 测试过期失效
}

#[tokio::test]
async fn test_inline_storage() {
    // 测试内联存储（< 4KB）
}

#[tokio::test]
async fn test_file_storage() {
    // 测试文件存储（>= 4KB）
}

#[tokio::test]
async fn test_compression() {
    // 测试压缩功能
}
```

## 5.2 集成测试（续）

```rust
// tests/integration_test.rs

#[tokio::test]
async fn test_namespace_invalidation() {
    // 测试命名空间失效
}

#[tokio::test]
async fn test_vary_cache() {
    // 测试 Vary 缓存
}

#[tokio::test]
async fn test_fetch_protection() {
    // 测试回源保护（防止并发击穿）
}

#[tokio::test]
async fn test_stats_tracking() {
    // 测试统计功能
}

#[tokio::test]
async fn test_index_persistence() {
    // 测试索引持久化和恢复
}

```

## 5.3 并发测试

```rust
// tests/concurrent_test.rs

#[tokio::test]
async fn test_concurrent_reads() {
    // 1000 并发读取
}

#[tokio::test]
async fn test_concurrent_writes() {
    // 100 并发写入同一键
}

#[tokio::test]
async fn test_concurrent_mixed() {
    // 读写混合场景
}

#[tokio::test]
async fn test_race_condition() {
    // 竞态条件测试
}

```

## 5.4 性能基准测试

```
// benches/benchmark.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_inline_read(c: &mut Criterion) {
    // 基准测试：内联存储读取
}

fn bench_file_read(c: &mut Criterion) {
    // 基准测试：文件存储读取
}

fn bench_write_performance(c: &mut Criterion) {
    // 基准测试：写入性能
}

criterion_group!(benches, bench_inline_read, bench_file_read, bench_write_performance);
criterion_main!(benches);
```

## 八、开发检查清单

8.1 开始前的准备

```
✅ 环境准备
├── [ ] Rust 工具链安装（rustc 1.70+）
├── [ ] IDE 配置（VS Code + rust-analyzer）
├── [ ] Git 仓库创建
└── [ ] 依赖项确认

✅ 文档准备
├── [ ] 架构设计文档（你已有）
├── [ ] API 接口定义（见上方）
├── [ ] 配置规范（见上方）
└── [ ] 错误类型定义（见上方）

✅ 项目结构
├── [ ] 按照建议的目录结构创建文件
├── [ ] Cargo.toml 配置完成
└── [ ] README.md 初始化
```

8.2 开发阶段检查

```
✅ 第一周（核心功能）
├── [ ] CacheMetadata 结构实现
├── [ ] Cache 基础实现（get/set）
├── [ ] 内联存储实现
├── [ ] 文件存储实现
├── [ ] LRU 淘汰实现
├── [ ] 过期检查实现
└── [ ] 单元测试（覆盖率 > 80%）

✅ 第二周（高级功能）
├── [ ] 命名空间/标签实现
├── [ ] 压缩功能实现
├── [ ] 回源保护实现
├── [ ] 并发写保护实现
├── [ ] 统计功能实现
└── [ ] 集成测试

✅ 第三周（优化和完善）
├── [ ] Vary 缓存实现
├── [ ] 索引持久化实现
├── [ ] 性能优化
├── [ ] 监控集成
├── [ ] 文档完善
└── [ ] 性能基准测试

✅ 第四周（生产准备）
├── [ ] 压力测试
├── [ ] 内存泄漏检查
├── [ ] 并发测试
├── [ ] 文档审查
└── [ ] 示例代码完善
```

## 十、示例代码

### 10.1 基础使用示例

```rust
// examples/basic.rs

use lightweight_cache::{Cache, CacheConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建缓存实例
    let cache = Cache::new(CacheConfig::default());
    
    // 2. 设置缓存
    cache.set(
        "user:123".to_string(),
        b"Alice".to_vec(),
        "text/plain".to_string(),
    ).await?;
    
    // 3. 读取缓存
    if let Some(data) = cache.get("user:123").await? {
        println!("User: {}", String::from_utf8_lossy(&data));
    }
    
    // 4. 失效缓存
    cache.invalidate("user:123").await?;
    
    // 5. 查看统计
    let stats = cache.statistics().await;
    println!("Hit rate: {:.2}%", stats.hit_rate * 100.0);
    
    Ok(())
}

```

### 10.2 带命名空间的示例

```rust
// examples/namespace.rs

use lightweight_cache::{Cache, CacheConfig, NamespaceInfo, CachePriority};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());
    
    // 设置博客文章缓存
    cache.set_with_namespace(
        "blog:post:123".to_string(),
        b"<html>Post 123</html>".to_vec(),
        "text/html".to_string(),
        NamespaceInfo {
            namespace: "blog".to_string(),
            tags: vec!["post:123".to_string(), "category:tech".to_string()],
            priority: CachePriority::Normal,
        },
    ).await?;
    
    // 失效整个 blog 命名空间
    let count = cache.invalidate_namespace("blog").await?;
    println!("Invalidated {} entries", count);
    
    // 失效特定标签
    let count = cache.invalidate_tags(&["category:tech".to_string()]).await?;
    println!("Invalidated {} entries by tag", count);
    
    Ok(())
}

```

### 10.3 Vary 缓存示例

```rust
// examples/vary.rs

use lightweight_cache::{Cache, VaryCondition, VaryValues};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());
    
    // 设置 Vary 缓存（根据语言和设备）
    cache.set_with_vary(
        "page:home".to_string(),
        b"<html lang='en'>Mobile Home</html>".to_vec(),
        "text/html".to_string(),
        vec![
            VaryCondition::AcceptLanguage,
            VaryCondition::UserAgent,
        ],
    ).await?;
    
    // 获取时需要提供 Vary 值
    let vary_values = VaryValues {
        language: Some("en".to_string()),
        encoding: None,
        user_agent: Some("mobile".to_string()),
        custom: HashMap::new(),
    };
    
    if let Some(data) = cache.get_vary("page:home", &vary_values).await? {
        println!("Got: {}", String::from_utf8_lossy(&data));
    }
    
    Ok(())
}

```

### 10.4 Axum 集成示例

```rust
// examples/axum_integration.rs

use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use lightweight_cache::Cache;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    cache: Arc<Cache>,
}

#[tokio::main]
async fn main() {
    let cache = Arc::new(Cache::new(CacheConfig::default()));
    let state = AppState { cache };
    
    let app = Router::new()
        .route("/blog/post/:id", get(show_post))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn show_post(
    State(state): State<AppState>,
    Path(post_id): Path<u64>,
) -> Html<String> {
    let cache_key = format!("blog:post:{}", post_id);
    
    // 尝试从缓存获取
    if let Ok(Some(cached)) = state.cache.get(&cache_key).await {
        return Html(String::from_utf8_lossy(&cached).to_string());
    }
    
    // 缓存未命中，生成内容
    let html = format!("<html><body><h1>Post {}</h1></body></html>", post_id);
    
    // 写入缓存
    let _ = state.cache.set(
        cache_key,
        html.as_bytes().to_vec(),
        "text/html".to_string(),
    ).await;
    
    Html(html)
}

```

## 十一、调试和排查工具

11.1 调试模式

```rust
// src/lib.rs

pub struct Cache {
    inner: Arc<CacheInner>,
    debug: bool,
}

impl Cache {
    pub fn with_debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }
    
    async fn debug_log(&self, message: &str) {
        if self.debug {
            tracing::debug!("[Cache] {}", message);
        }
    }
}

// 使用
let cache = Cache::new(config).with_debug(true);
```

11.2 统计报告

```rust
// 生成详细的统计报告
pub async fn generate_report(&self) -> String {
    let stats = self.statistics().await;
    
    format!(
        r#"
=== Cache Statistics Report ===

Performance:
- Total Requests: {}
- Cache Hits: {}
- Cache Misses: {}
- Hit Rate: {:.2}%

Capacity:
- Total Entries: {}
- Memory Usage: {:.2} MB
- Disk Usage: {:.2} GB

Hot Keys (Top 10):
{}
        "#,
        stats.total_requests,
        stats.hits,
        stats.misses,
        stats.hit_rate * 100.0,
        stats.total_entries,
        stats.memory_usage_mb,
        stats.disk_usage_gb,
        self.format_hot_keys().await,
    )
}

```



