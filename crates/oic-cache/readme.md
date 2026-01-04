# oic-cache

轻量级索引 + 文件存储缓存系统，内存只存索引元数据。

## 功能特性

- ✅ 数据大小阈值（4KB）- 小数据内联存储，大数据文件存储
- ✅ 命名空间/分组支持
- ✅ Vary 变种缓存
- ✅ 回源保护（防击穿）
- ✅ 统计和热度追踪
- ✅ 并发写支持
- ✅ 序列化支持
- ✅ 压缩支持（Gzip, Zstd, Brotli）
- ✅ LRU 淘汰策略
- ✅ 过期时间管理

## 快速开始

### 基础使用

```rust
use oic_cache::{Cache, CacheConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建缓存实例
    let cache = Cache::new(CacheConfig::default());
    
    // 设置缓存
    cache.set(
        "user:123".to_string(),
        b"Alice".to_vec(),
        "text/plain".to_string(),
    ).await?;
    
    // 读取缓存
    if let Some(data) = cache.get("user:123").await? {
        println!("User: {}", String::from_utf8_lossy(&data));
    }
    
    Ok(())
}
```

### 索引持久化和自动加载

缓存支持将索引保存到磁盘，并在重启后自动恢复：

```rust
use oic_cache::{Cache, CacheConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = CacheConfig::default();
    config.storage.auto_load_index = true; // 启用自动加载
    
    // 创建缓存并自动加载索引（如果存在）
    let cache = Cache::new_with_auto_load(config).await?;
    
    // 使用缓存...
    cache.set("key".to_string(), b"value".to_vec(), "text/plain".to_string()).await?;
    
    // 保存索引到磁盘（通常在应用关闭时调用）
    cache.save_index().await?;
    
    Ok(())
}
```

**注意：**
- `Cache::new()` 是同步的，不会自动加载索引
- `Cache::new_with_auto_load()` 是异步的，会自动加载已保存的索引
- `Cache::from_config_file()` 也会自动加载索引

## 更多示例

查看 `examples/` 目录获取更多使用示例。

## 运行测试

```shell
# 运行所有测试
cargo test --package oic_cache

# 运行集成测试
cargo test --package oic_cache --test integration_test

# 运行并发测试
cargo test --package oic_cache --test concurrent_test

# 运行 Axum 集成示例
cargo run --package oic_cache --example axum_integration
```
