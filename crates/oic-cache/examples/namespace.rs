use oic_cache::{Cache, CacheConfig, CachePriority, NamespaceInfo};
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());

    // 设置博客文章缓存
    cache
        .set_with_namespace(
            "blog:post:123".to_string(),
            Bytes::copy_from_slice(b"<html>Post 123</html>"),
            "text/html".to_string(),
            NamespaceInfo {
                namespace: "blog".to_string(),
                tags: vec!["post:123".to_string(), "category:tech".to_string()],
                priority: CachePriority::Normal,
            },
        )
        .await?;

    cache
        .set_with_namespace(
            "blog:post:456".to_string(),
            Bytes::copy_from_slice(b"<html>Post 456</html>"),
            "text/html".to_string(),
            NamespaceInfo {
                namespace: "blog".to_string(),
                tags: vec!["post:456".to_string(), "category:tech".to_string()],
                priority: CachePriority::High,
            },
        )
        .await?;

    // 失效整个 blog 命名空间
    let count = cache.invalidate_namespace("blog").await?;
    println!("Invalidated {} entries", count);

    // 重新设置
    cache
        .set_with_namespace(
            "blog:post:123".to_string(),
            Bytes::copy_from_slice(b"<html>Post 123</html>"),
            "text/html".to_string(),
            NamespaceInfo {
                namespace: "blog".to_string(),
                tags: vec!["post:123".to_string(), "category:tech".to_string()],
                priority: CachePriority::Normal,
            },
        )
        .await?;

    cache
        .set_with_namespace(
            "blog:post:456".to_string(),
            Bytes::copy_from_slice(b"<html>Post 456</html>"),
            "text/html".to_string(),
            NamespaceInfo {
                namespace: "blog".to_string(),
                tags: vec!["post:456".to_string(), "category:tech".to_string()],
                priority: CachePriority::High,
            },
        )
        .await?;

    // 失效特定标签
    let count = cache
        .invalidate_tags(&["category:tech".to_string()])
        .await?;
    println!("Invalidated {} entries by tag", count);

    Ok(())
}

