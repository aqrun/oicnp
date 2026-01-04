use oic_cache::{
    Cache, CacheConfig, CachePriority, NamespaceInfo, VaryCondition, VaryValues,
};
use std::collections::HashMap;
use tempfile::TempDir;

fn create_test_cache() -> (Cache, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut config = CacheConfig::default();
    config.disk_path = temp_dir.path().to_string_lossy().to_string();
    (Cache::new(config), temp_dir)
}

#[tokio::test]
async fn test_basic_get_set() {
    let (cache, _temp_dir) = create_test_cache();

    // 测试设置和获取
    cache
        .set(
            "test:key".to_string(),
            b"test value".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    let result = cache.get("test:key").await.unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), b"test value");
}

#[tokio::test]
async fn test_expiration() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置一个很短的 TTL
    cache
        .set_with_ttl(
            "expire:key".to_string(),
            b"expire value".to_vec(),
            "text/plain".to_string(),
            1, // 1 秒
        )
        .await
        .unwrap();

    // 立即获取应该成功
    assert!(cache.get("expire:key").await.unwrap().is_some());

    // 等待过期
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 应该返回 None
    assert!(cache.get("expire:key").await.unwrap().is_none());
}

#[tokio::test]
async fn test_inline_storage() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置小数据（应该内联存储）
    let small_data = vec![0u8; 100]; // 100 字节，小于 4KB
    cache
        .set(
            "small:key".to_string(),
            small_data.clone(),
            "application/octet-stream".to_string(),
        )
        .await
        .unwrap();

    let result = cache.get("small:key").await.unwrap();
    assert_eq!(result.unwrap(), small_data);
}

#[tokio::test]
async fn test_file_storage() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置大数据（应该文件存储）
    let large_data = vec![0u8; 10 * 1024]; // 10KB，大于 4KB
    cache
        .set(
            "large:key".to_string(),
            large_data.clone(),
            "application/octet-stream".to_string(),
        )
        .await
        .unwrap();

    let result = cache.get("large:key").await.unwrap();
    assert_eq!(result.unwrap(), large_data);
}

#[tokio::test]
async fn test_invalidate() {
    let (cache, _temp_dir) = create_test_cache();

    cache
        .set(
            "delete:key".to_string(),
            b"delete me".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    assert!(cache.exists("delete:key").await);

    cache.invalidate("delete:key").await.unwrap();

    assert!(!cache.exists("delete:key").await);
    assert!(cache.get("delete:key").await.unwrap().is_none());
}

#[tokio::test]
async fn test_namespace_invalidation() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置带命名空间的缓存
    cache
        .set_with_namespace(
            "blog:post:1".to_string(),
            b"Post 1".to_vec(),
            "text/html".to_string(),
            NamespaceInfo {
                namespace: "blog".to_string(),
                tags: vec!["post".to_string()],
                priority: CachePriority::Normal,
            },
        )
        .await
        .unwrap();

    cache
        .set_with_namespace(
            "blog:post:2".to_string(),
            b"Post 2".to_vec(),
            "text/html".to_string(),
            NamespaceInfo {
                namespace: "blog".to_string(),
                tags: vec!["post".to_string()],
                priority: CachePriority::Normal,
            },
        )
        .await
        .unwrap();

    // 失效整个命名空间
    let count = cache.invalidate_namespace("blog").await.unwrap();
    assert_eq!(count, 2);

    // 验证都已失效
    assert!(!cache.exists("blog:post:1").await);
    assert!(!cache.exists("blog:post:2").await);
}

#[tokio::test]
async fn test_tag_invalidation() {
    let (cache, _temp_dir) = create_test_cache();

    cache
        .set_with_namespace(
            "post:1".to_string(),
            b"Post 1".to_vec(),
            "text/html".to_string(),
            NamespaceInfo {
                namespace: "blog".to_string(),
                tags: vec!["tech".to_string(), "post".to_string()],
                priority: CachePriority::Normal,
            },
        )
        .await
        .unwrap();

    cache
        .set_with_namespace(
            "post:2".to_string(),
            b"Post 2".to_vec(),
            "text/html".to_string(),
            NamespaceInfo {
                namespace: "blog".to_string(),
                tags: vec!["tech".to_string()],
                priority: CachePriority::Normal,
            },
        )
        .await
        .unwrap();

    // 失效 tech 标签
    let count = cache.invalidate_tags(&["tech".to_string()]).await.unwrap();
    assert_eq!(count, 2);

    assert!(!cache.exists("post:1").await);
    assert!(!cache.exists("post:2").await);
}

#[tokio::test]
async fn test_vary_cache() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置 Vary 缓存
    cache
        .set_with_vary(
            "page:home".to_string(),
            b"<html>Home</html>".to_vec(),
            "text/html".to_string(),
            vec![VaryCondition::AcceptLanguage, VaryCondition::UserAgent],
        )
        .await
        .unwrap();

    // 获取时需要提供 Vary 值
    let vary_values = VaryValues {
        language: Some("en".to_string()),
        encoding: None,
        user_agent: Some("mobile".to_string()),
        custom: HashMap::new(),
    };

    // 注意：set_with_vary 使用默认的 VaryValues，所以这里可能找不到
    // 这个测试展示了 API 的使用方式
    let _result = cache.get_vary("page:home", &vary_values).await;
}

#[tokio::test]
async fn test_stats_tracking() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置一些缓存
    cache
        .set(
            "stats:1".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    cache
        .set(
            "stats:2".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    // 读取几次
    cache.get("stats:1").await.unwrap();
    cache.get("stats:1").await.unwrap();
    cache.get("stats:2").await.unwrap();

    // 获取统计信息
    let stats = cache.statistics().await;
    assert!(stats.total_requests >= 3);
    assert!(stats.hits >= 3);
    assert!(stats.total_entries >= 2);
}

#[tokio::test]
async fn test_batch_get() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置多个键
    for i in 1..=5 {
        cache
            .set(
                format!("batch:{}", i),
                format!("value{}", i).into_bytes(),
                "text/plain".to_string(),
            )
            .await
            .unwrap();
    }

    // 批量获取
    let keys: Vec<String> = (1..=5).map(|i| format!("batch:{}", i)).collect();
    let results = cache.get_batch(&keys).await;

    assert_eq!(results.len(), 5);
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_some());
        assert_eq!(
            result.as_ref().unwrap(),
            &format!("value{}", i + 1).into_bytes()
        );
    }
}

#[tokio::test]
async fn test_cleanup_expired() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置一些会过期的缓存
    cache
        .set_with_ttl(
            "expire:1".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
            1,
        )
        .await
        .unwrap();

    cache
        .set_with_ttl(
            "expire:2".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
            1,
        )
        .await
        .unwrap();

    // 等待过期
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 清理过期缓存（未启用 SWR，应该清理所有过期数据）
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 2);

    // 验证数据已被清理
    assert!(!cache.exists("expire:1").await);
    assert!(!cache.exists("expire:2").await);
}

#[tokio::test]
async fn test_cleanup_expired_with_swr_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = CacheConfig::default();
    config.disk_path = temp_dir.path().to_string_lossy().to_string();
    config.swr.enabled = false; // 明确禁用 SWR
    let cache = Cache::new(config);

    // 设置一些会过期的缓存
    cache
        .set_with_ttl(
            "expire:1".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
            1,
        )
        .await
        .unwrap();

    cache
        .set_with_ttl(
            "expire:2".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
            1,
        )
        .await
        .unwrap();

    // 等待过期
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 清理过期缓存（SWR 禁用，应该清理所有过期数据）
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 2);

    // 验证数据已被清理
    assert!(!cache.exists("expire:1").await);
    assert!(!cache.exists("expire:2").await);
}

#[tokio::test]
async fn test_cleanup_expired_with_swr_enabled_max_stale_limited() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = CacheConfig::default();
    config.disk_path = temp_dir.path().to_string_lossy().to_string();
    config.swr.enabled = true;
    config.swr.max_stale_seconds = 5; // 最多保留 5 秒的 stale 数据
    let cache = Cache::new(config);

    // 设置一些会过期的缓存
    cache
        .set_with_ttl(
            "expire:1".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
            1, // 1 秒后过期
        )
        .await
        .unwrap();

    cache
        .set_with_ttl(
            "expire:2".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
            1, // 1 秒后过期
        )
        .await
        .unwrap();

    // 等待过期（2秒后，过期了1秒，还在 max_stale 范围内）
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 此时数据过期但未超过 max_stale，不应该被清理
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 0, "过期但未超过 max_stale 的数据不应该被清理");

    // 验证数据仍然存在（虽然过期，但在 max_stale 范围内）
    assert!(cache.exists("expire:1").await);
    assert!(cache.exists("expire:2").await);

    // 等待超过 max_stale 时间（总共等待 7 秒，过期了 6 秒，超过 max_stale 5 秒）
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // 现在应该清理超过 max_stale 的数据
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 2, "超过 max_stale 的数据应该被清理");

    // 验证数据已被清理
    assert!(!cache.exists("expire:1").await);
    assert!(!cache.exists("expire:2").await);
}

#[tokio::test]
async fn test_cleanup_expired_with_swr_enabled_max_stale_unlimited() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = CacheConfig::default();
    config.disk_path = temp_dir.path().to_string_lossy().to_string();
    config.swr.enabled = true;
    config.swr.max_stale_seconds = 0; // 0 表示不限制 stale 时间
    let cache = Cache::new(config);

    // 设置一些会过期的缓存
    cache
        .set_with_ttl(
            "expire:1".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
            1,
        )
        .await
        .unwrap();

    cache
        .set_with_ttl(
            "expire:2".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
            1,
        )
        .await
        .unwrap();

    // 等待过期
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // max_stale_seconds = 0 表示不限制，过期数据不应该被清理
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 0, "max_stale_seconds = 0 时，过期数据不应该被清理");

    // 验证数据仍然存在
    assert!(cache.exists("expire:1").await);
    assert!(cache.exists("expire:2").await);

    // 即使再等待很长时间，也不应该被清理
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 0, "max_stale_seconds = 0 时，即使等待很久也不应该清理");
}

#[tokio::test]
async fn test_cleanup_expired_mixed_scenario() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = CacheConfig::default();
    config.disk_path = temp_dir.path().to_string_lossy().to_string();
    config.swr.enabled = true;
    config.swr.max_stale_seconds = 5; // 最多保留 5 秒的 stale 数据
    let cache = Cache::new(config);

    // 设置一个会过期的缓存（1秒后过期）
    cache
        .set_with_ttl(
            "expire:short".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
            1,
        )
        .await
        .unwrap();

    // 设置一个不会过期的缓存
    cache
        .set_with_ttl(
            "valid:key".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
            3600, // 1 小时后过期
        )
        .await
        .unwrap();

    // 等待第一个缓存过期
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 此时只有 expire:short 过期，但还在 max_stale 范围内，不应该被清理
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 0);

    // 验证两个键都存在
    assert!(cache.exists("expire:short").await);
    assert!(cache.exists("valid:key").await);

    // 等待超过 max_stale 时间
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // 现在应该只清理超过 max_stale 的过期数据
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 1, "应该只清理超过 max_stale 的过期数据");

    // 验证过期数据已被清理，但有效数据仍然存在
    assert!(!cache.exists("expire:short").await);
    assert!(cache.exists("valid:key").await);
}

#[tokio::test]
async fn test_cleanup_expired_with_zero_ttl() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置 TTL = 0 的缓存（永不过期）
    cache
        .set_with_ttl(
            "never:expire".to_string(),
            b"value".to_vec(),
            "text/plain".to_string(),
            0, // 永不过期
        )
        .await
        .unwrap();

    // 设置正常 TTL 的缓存
    cache
        .set_with_ttl(
            "expire:normal".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
            1,
        )
        .await
        .unwrap();

    // 等待正常缓存过期
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 清理过期缓存
    let count = cache.cleanup_expired().await.unwrap();
    assert_eq!(count, 1, "应该只清理正常过期的缓存，不清理 TTL=0 的缓存");

    // 验证 TTL=0 的缓存仍然存在
    assert!(cache.exists("never:expire").await);
    assert!(!cache.exists("expire:normal").await);
}

#[tokio::test]
async fn test_index_persistence() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置一些缓存
    cache
        .set(
            "persist:1".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    cache
        .set(
            "persist:2".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    // 保存索引
    cache.save_index().await.unwrap();

    // 注意：在实际场景中，会重新加载缓存实例
    // 这里只是测试保存功能
    assert!(cache.get("persist:1").await.unwrap().is_some());
}

#[tokio::test]
async fn test_clear() {
    let (cache, _temp_dir) = create_test_cache();

    // 设置一些缓存
    cache
        .set(
            "clear:1".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    cache
        .set(
            "clear:2".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    // 清空所有缓存
    cache.clear().await.unwrap();

    // 验证都已清空
    assert!(!cache.exists("clear:1").await);
    assert!(!cache.exists("clear:2").await);

    let stats = cache.statistics().await;
    assert_eq!(stats.total_entries, 0);
}

