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

## 更多示例

查看 `examples/` 目录获取更多使用示例。
