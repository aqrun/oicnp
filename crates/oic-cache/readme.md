# oic-cache

轻量级索引 + redb 持久化缓存系统，内存只存热点索引元数据，数据持久化在单文件数据库中。

## 功能特性

- ✅ **智能存储策略** - 小数据（<4KB）内联存储，大数据持久化到 redb
- ✅ **命名空间/分组支持** - 逻辑分组和批量失效
- ✅ **Vary 变种缓存** - 基于请求头的多版本缓存
- ✅ **回源保护（防击穿）** - 防止缓存击穿和缓存雪崩
- ✅ **统计和热度追踪** - 访问统计、命中率、热点键追踪
- ✅ **并发安全** - 支持高并发读写
- ✅ **自动序列化/反序列化** - 通过 `CacheExt` trait 支持
- ✅ **压缩支持** - Gzip, Zstd, Brotli 压缩算法
- ✅ **LRU 淘汰策略** - 内存索引 LRU 管理
- ✅ **过期时间管理** - TTL 和过期检查
- ✅ **Stale-While-Revalidate (SWR)** - 过期缓存兜底策略
- ✅ **索引持久化** - 索引元数据持久化到 redb，支持自动加载
- ✅ **自动保存** - 支持定期保存和更新后延迟保存（debounce）
- ✅ **Loco_rs 兼容** - 提供与 loco_rs cache trait 兼容的 API
- ✅ **独立服务模式** - 支持 Redis 协议（6379）与 gRPC（50051），多进程共享同一缓存

## 架构设计

### 核心设计理念

- **内存只存热点索引**：内存中维护 `CacheMetadata` 的 LRU 索引用于快速命中
- **统一持久化后端**：索引与大对象数据都持久化到 `disk_path/cache.redb`
- **智能存储策略**：根据数据大小自动选择内联存储或 redb 存储
- **扩展 Trait 模式**：核心 `Cache` 只处理 `Vec<u8>`，序列化逻辑通过 `CacheExt` trait 提供

### 存储策略

- **内联存储**（< 4KB）：数据直接存储在元数据中，零文件 I/O
- **redb 存储**（>= 4KB）：数据存储在 `cache.redb` 中，支持压缩

## 快速开始

### 安装

```toml
[dependencies]
oic_cache = { path = "../oic-cache" }
```

### 基础使用（核心 API）

核心 `Cache` API 只处理原始字节（`Vec<u8>`）：

```rust
use oic_cache::{Cache, CacheConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建缓存实例
    let cache = Cache::new(CacheConfig::default());
    
    // 设置缓存（原始字节）
    cache.set(
        "user:123".to_string(),
        b"Alice".to_vec(),
        "text/plain".to_string(),
    ).await?;
    
    // 读取缓存（返回原始字节）
    if let Some(data) = cache.get("user:123").await? {
        println!("User: {}", String::from_utf8_lossy(&data));
    }
    
    // 设置带过期时间的缓存
    cache.set_with_ttl(
        "session:abc".to_string(),
        b"session_data".to_vec(),
        "application/json".to_string(),
        3600, // TTL: 1 小时
    ).await?;
    
    // 删除缓存
    cache.invalidate("user:123").await?;
    
    Ok(())
}
```

### 扩展 API（自动序列化）

通过 `CacheExt` trait 使用自动序列化/反序列化：

```rust
use oic_cache::{Cache, CacheConfig, CacheExt};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());
    
    // 插入序列化对象（自动序列化为 JSON）
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    cache.insert("user:1", &user).await?;
    
    // 获取并自动反序列化
    let retrieved: Option<User> = cache.get("user:1").await?;
    println!("User: {:?}", retrieved);
    
    // 带过期时间的插入
    cache.insert_with_expiry("user:2", &user, Duration::from_secs(300)).await?;
    
    // 获取或插入（lazy loading）
    let user3 = cache.get_or_insert("user:3", async {
        Ok(User {
            id: 3,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        })
    }).await?;
    
    Ok(())
}
```

### Axum 便捷方法

`CacheExt` 还提供了 Axum 专用的便捷方法：

```rust
use oic_cache::{Cache, CacheConfig, CacheExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());
    
    // 设置 JSON 缓存
    let user = serde_json::json!({
        "id": 1,
        "name": "Alice"
    });
    cache.set_json("user:1", &user, 3600).await?;
    
    // 获取 JSON 缓存
    let user: Option<serde_json::Value> = cache.get_json("user:1").await?;
    
    // 设置 HTML 缓存
    cache.set_html("page:home", "<h1>Hello</h1>", 3600).await?;
    
    // 获取 HTML 缓存
    let html: Option<String> = cache.get_html("page:home").await?;
    
    Ok(())
}
```

## 独立服务模式

oic-cache 可作为独立进程运行，对外提供 **Redis 协议**（供 oic-web 等高频访问）和 **gRPC**（供 oic-admin 等管理端）。多应用共享同一份缓存。

### 启动服务

编译并运行独立服务二进制：

```bash
cargo build --release -p oic_cache --bin oic-cache-server
./target/release/oic-cache-server
```

**监听地址** 可在 `CacheConfig` 的 `[server]` 中配置（与 `disk_path`、`storage` 等同文件），环境变量会覆盖配置文件：

| 来源 | 说明 |
|------|------|
| 环境变量 | `OIC_CACHE_REDIS_ADDR`、`OIC_CACHE_GRPC_ADDR`（覆盖 config） |
| 配置文件 | `OIC_CACHE_CONFIG` 指向的 TOML 中 `[server]` 的 `redis_addr`、`grpc_addr` |
| 默认值 | `0.0.0.0:6379`、`0.0.0.0:50051` |

配置文件示例（`cache.toml`）：

```toml
[server]
redis_addr = "0.0.0.0:6379"
grpc_addr = "0.0.0.0:50051"
```

命令行用环境变量覆盖配置：

```bash
OIC_CACHE_CONFIG=/etc/oic/cache.toml OIC_CACHE_REDIS_ADDR=0.0.0.0:6380 ./target/release/oic-cache-server
```

启动后会自动尝试从配置中的 `disk_path/cache.redb` 加载已有索引；未设置 `OIC_CACHE_CONFIG` 时使用默认 `disk_path`。

### 客户端接入 Redis 协议

任何兼容 Redis 协议的客户端均可连接，例如 oic-web 使用 [redis](https://crates.io/crates/redis)：

```toml
[dependencies]
redis = { version = "0.27", features = ["tokio-comp", "disable-client-setinfo"] }
```

```rust
use redis::AsyncCommands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut conn = client.get_multiplexed_async_connection().await?;

    // 与使用 Redis 一致
    redis::cmd("SET").arg("user:123").arg(b"alice").query_async::<()>(&mut conn).await?;
    let value: Vec<u8> = redis::cmd("GET").arg("user:123").query_async(&mut conn).await?;

    // 带过期时间
    redis::cmd("SET").arg("session:abc").arg(b"data").arg("EX").arg(3600i64).query_async::<()>(&mut conn).await?;

    // 自定义命令：按命名空间失效、查看统计
    let _: i64 = redis::cmd("INVALIDATE_NS").arg("users").query_async(&mut conn).await?;
    let stats: String = redis::cmd("STATS").query_async(&mut conn).await?;

    Ok(())
}
```

**服务端支持的 Redis 命令：** `GET`、`SET`（支持 `EX seconds`）、`DEL`、`EXISTS`、`FLUSHALL`、`PING`；扩展命令 `INVALIDATE_NS namespace`、`STATS`。

### 客户端接入 gRPC

管理端（如 oic-admin）可通过 gRPC 调用统计、按命名空间失效等。需使用与 `proto/cache.proto` 同源的客户端代码（例如本 crate 的 `oic_cache::server::proto` 或自行用 tonic 从同一 proto 生成）：

```rust
use oic_cache::server::proto::cache_service_client::CacheServiceClient;
use oic_cache::server::proto::{Empty, GetRequest, InvalidateRequest, SetRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = CacheServiceClient::connect("http://127.0.0.1:50051").await?;

    // 统计信息
    let stats = client.get_statistics(Empty {}).await?.into_inner();
    println!("hits: {}, misses: {}, hit_rate: {}", stats.hits, stats.misses, stats.hit_rate);

    // 按命名空间失效
    let inv = client
        .invalidate_namespace(InvalidateRequest {
            namespace: "users".to_string(),
        })
        .await?
        .into_inner();
    println!("invalidated: {}", inv.invalidated_count);

    // 读写（与 Redis 共用同一缓存）
    client
        .set(SetRequest {
            key: "grpc:key".to_string(),
            data: b"value".to_vec(),
            ttl_seconds: 300,
        })
        .await?;
    let res = client.get(GetRequest { key: "grpc:key".to_string() }).await?.into_inner();
    assert!(res.found);

    Ok(())
}
```

gRPC 接口定义见 `proto/cache.proto`，包含 `Get`、`Set`、`GetStatistics`、`InvalidateNamespace`。

---

## 高级功能

### redb 持久化和自动加载

缓存将索引元数据与大对象数据持久化到 `disk_path/cache.redb`，并在重启后自动恢复：

```rust
use oic_cache::{Cache, CacheConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = CacheConfig::default();
    config.storage.auto_load_index = true; // 启用自动加载（从 redb）
    config.storage.auto_save_interval_seconds = 30; // 每 30 秒定期保存
    config.storage.auto_save_debounce_ms = 2000; // 更新后延迟 2 秒保存
    
    // 创建缓存并自动加载持久化索引（如果存在）
    let cache = Cache::new_with_auto_load(config).await?;
    
    // 使用缓存...
    cache.set("key".to_string(), b"value".to_vec(), "text/plain".to_string()).await?;
    
    // 手动保存索引（写入 redb，可选）
    cache.save_index().await?;
    
    Ok(())
}
```

**自动保存机制：**
- **定期保存**：每 `auto_save_interval_seconds` 秒保存一次（兜底）
- **更新后延迟保存**：更新后延迟 `auto_save_debounce_ms` 毫秒保存（重置式 debounce）
- 两种机制结合，既保证数据安全，又减少 I/O 负载

### Stale-While-Revalidate (SWR)

支持过期缓存兜底策略，在数据过期时仍可返回过期数据，同时在后台重新获取：

```rust
use oic_cache::{Cache, CacheConfig, SwrConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = CacheConfig::default();
    // 启用 SWR
    config.swr.enabled = true;
    config.swr.max_stale_seconds = 3600; // 过期后最多保留 1 小时
    
    let cache = Cache::new(config);
    
    // 设置一个短 TTL 的缓存
    cache.set_with_ttl(
        "data:1".to_string(),
        b"important_data".to_vec(),
        "text/plain".to_string(),
        5, // 5 秒后过期
    ).await?;
    
    // 等待 6 秒后...
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
    
    // 仍然可以获取到数据（虽然已过期，但在 max_stale_seconds 内）
    if let Some(data) = cache.get("data:1").await? {
        println!("Got stale data: {}", String::from_utf8_lossy(&data));
    }
    
    Ok(())
}
```

### 命名空间和批量失效

```rust
use oic_cache::{Cache, CacheConfig, NamespaceInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());
    
    // 设置命名空间
    let namespace = NamespaceInfo {
        namespace: "users".to_string(),
        tags: vec!["v1".to_string(), "public".to_string()],
    };
    
    cache.set_with_namespace(
        "user:1".to_string(),
        b"user_data".to_vec(),
        "application/json".to_string(),
        namespace,
    ).await?;
    
    // 按命名空间失效
    cache.invalidate_namespace("users").await?;
    
    // 按标签失效
    cache.invalidate_tags(&["v1".to_string()]).await?;
    
    Ok(())
}
```

### Vary 变种缓存

支持基于请求头的多版本缓存：

```rust
use oic_cache::{Cache, CacheConfig, VaryCondition};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());
    
    // 设置 Vary 缓存（基于 Accept-Language）
    let vary_conditions = vec![VaryCondition {
        header: "Accept-Language".to_string(),
        values: vec!["en".to_string(), "zh".to_string()],
    }];
    
    cache.set_with_vary(
        "page:home".to_string(),
        b"english_content".to_vec(),
        "text/html".to_string(),
        vary_conditions,
    ).await?;
    
    // 使用 VaryValues 获取特定变种
    use oic_cache::VaryValues;
    let vary_values = VaryValues::from_headers(&[
        ("Accept-Language", "en"),
    ]);
    
    if let Some(data) = cache.get_vary("page:home", &vary_values).await? {
        println!("Content: {}", String::from_utf8_lossy(&data));
    }
    
    Ok(())
}
```

### 统计和监控

```rust
use oic_cache::{Cache, CacheConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());
    
    // 获取统计信息
    let stats = cache.get_statistics().await;
    println!("Hits: {}, Misses: {}", stats.hits(), stats.misses());
    println!("Hit rate: {:.2}%", stats.hit_rate() * 100.0);
    
    // 获取热点键
    let hot_keys = cache.get_hot_keys(10).await;
    for (key, count) in hot_keys {
        println!("Hot key: {} (accessed {} times)", key, count);
    }
    
    Ok(())
}
```

## 配置

### CacheConfig

```rust
use oic_cache::CacheConfig;

let mut config = CacheConfig::default();

// 存储配置
config.disk_path = "/tmp/cache".into();
config.storage.inline_threshold = 4096; // 4KB
config.storage.auto_load_index = true;
config.storage.auto_save_interval_seconds = 30;
config.storage.auto_save_debounce_ms = 2000;

// 压缩配置
config.compression.enabled = true;
config.compression.default_algorithm = CompressionAlgorithm::Zstd;
config.compression.min_size = 1024; // 1KB

// LRU 配置
config.lru.max_size = 10000;

// SWR 配置
config.swr.enabled = true;
config.swr.max_stale_seconds = 3600;

// 默认 TTL
config.default_ttl_seconds = 3600;
```

### 从配置文件加载

```rust
use oic_cache::Cache;

// 从 TOML 配置文件加载
let cache = Cache::from_config_file("cache.toml").await?;
```

## API 参考

### 核心 Cache API

核心 `Cache` 只处理 `Vec<u8>`：

- `get(key: &str) -> Result<Option<Vec<u8>>>` - 获取缓存
- `set(key: String, data: Vec<u8>, content_type: String) -> Result<()>` - 设置缓存
- `set_with_ttl(key: String, data: Vec<u8>, content_type: String, ttl_seconds: i64) -> Result<()>` - 设置带过期时间的缓存
- `invalidate(key: &str) -> Result<()>` - 删除缓存
- `clear() -> Result<()>` - 清空所有缓存
- `exists(key: &str) -> bool` - 检查键是否存在
- `get_statistics() -> CacheStatistics` - 获取统计信息

### CacheExt API

`CacheExt` trait 提供自动序列化/反序列化：

**Loco_rs 兼容方法：**
- `ping() -> Result<()>` - 健康检查
- `contains_key(key: &str) -> Result<bool>` - 检查键是否存在
- `get<T>(key: &str) -> Result<Option<T>>` - 获取并反序列化
- `insert<T>(key: &str, value: &T) -> Result<()>` - 插入并序列化
- `insert_with_expiry<T>(key: &str, value: &T, duration: Duration) -> Result<()>` - 带过期时间的插入
- `get_or_insert<T, F>(key: &str, f: F) -> Result<T>` - 获取或插入
- `get_or_insert_with_expiry<T, F>(key: &str, duration: Duration, f: F) -> Result<T>` - 带过期时间的获取或插入
- `remove(key: &str) -> Result<()>` - 删除键
- `clear() -> Result<()>` - 清空缓存

**Axum 便捷方法：**
- `set_json<T>(key: String, value: &T, ttl_seconds: i64) -> Result<()>` - 设置 JSON 缓存
- `get_json<T>(key: &str) -> Result<Option<T>>` - 获取 JSON 缓存
- `set_html(key: String, html: &str, ttl_seconds: i64) -> Result<()>` - 设置 HTML 缓存
- `get_html(key: &str) -> Result<Option<String>>` - 获取 HTML 缓存
- `set_json_with_namespace<T>(...) -> Result<()>` - 带命名空间的 JSON 缓存
- `set_json_with_vary<T>(...) -> Result<()>` - 带 Vary 的 JSON 缓存

## 示例

查看 `examples/` 目录获取更多使用示例：

```bash
# 基础使用示例
cargo run --package oic_cache --example basic

# 持久化示例
cargo run --package oic_cache --example persistence

# SWR 示例
cargo run --package oic_cache --example swr

# 命名空间示例
cargo run --package oic_cache --example namespace

# Vary 缓存示例
cargo run --package oic_cache --example vary

# Loco_rs 兼容 API 示例
cargo run --package oic_cache --example loco_compat

# Axum 集成示例
cargo run --package oic_cache --example axum_integration

# Axum 扩展方法示例
cargo run --package oic_cache --example axum_ext
```

## 运行测试

```bash
# 运行所有测试
cargo test --package oic_cache

# 运行集成测试
cargo test --package oic_cache --test integration_test

# 运行并发测试
cargo test --package oic_cache --test concurrent_test

# 运行服务化集成测试（Redis + gRPC）
cargo test --package oic_cache --test server_integration_test
```

## 性能特性

- **内存占用低**：内存主要存热点元数据索引（LRU）
- **磁盘 I/O 优化**：小数据内联存储，减少落盘 I/O
- **并发安全**：使用 `DashMap` 和 `RwLock` 保证并发安全
- **压缩支持**：大数据自动压缩，节省磁盘空间
- **LRU 淘汰**：内存索引使用 LRU 策略，自动淘汰不常用项

## 设计原则

1. **关注点分离**：核心 `Cache` 只处理 `Vec<u8>`，序列化逻辑通过 `CacheExt` trait 提供
2. **内存优化**：内存存热点索引，持久化数据统一存储在 redb
3. **灵活扩展**：通过 trait 扩展，不污染核心 API
4. **向后兼容**：提供与 loco_rs 兼容的 API

