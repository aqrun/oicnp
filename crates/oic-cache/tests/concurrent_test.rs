use oic_cache::{Cache, CacheConfig};
use std::sync::Arc;
use tempfile::TempDir;

fn create_test_cache() -> (Cache, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut config = CacheConfig::default();
    config.disk_path = temp_dir.path().to_string_lossy().to_string();
    (Cache::new(config), temp_dir)
}

#[tokio::test]
async fn test_concurrent_reads() {
    let (cache, _temp_dir) = create_test_cache();
    let cache = Arc::new(cache);

    // 设置一个键
    cache
        .set(
            "concurrent:read".to_string(),
            b"test value".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    // 1000 个并发读取
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let cache = cache.clone();
        handles.push(tokio::spawn(async move {
            cache.get("concurrent:read").await
        }));
    }

    // 等待所有读取完成
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data, Some(b"test value".to_vec()));
    }
}

#[tokio::test]
async fn test_concurrent_writes() {
    let (cache, _temp_dir) = create_test_cache();
    let cache = Arc::new(cache);

    // 100 个并发写入同一键
    let mut handles = Vec::new();
    for i in 0..100 {
        let cache = cache.clone();
        let value = format!("value{}", i);
        handles.push(tokio::spawn(async move {
            cache
                .set(
                    "concurrent:write".to_string(),
                    value.into_bytes(),
                    "text/plain".to_string(),
                )
                .await
        }));
    }

    // 等待所有写入完成
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    // 验证最后的值存在（可能是任意一个写入的值）
    let result = cache.get("concurrent:write").await.unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn test_concurrent_mixed() {
    let (cache, _temp_dir) = create_test_cache();
    let cache = Arc::new(cache);

    // 初始设置
    cache
        .set(
            "mixed:key".to_string(),
            b"initial".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    // 混合读写操作
    let mut read_handles = Vec::new();
    let mut write_handles = Vec::new();

    // 50 个读取任务
    for _ in 0..50 {
        let cache = cache.clone();
        read_handles.push(tokio::spawn(async move {
            cache.get("mixed:key").await
        }));
    }

    // 50 个写入任务
    for i in 0..50 {
        let cache = cache.clone();
        let value = format!("update{}", i);
        write_handles.push(tokio::spawn(async move {
            cache
                .set(
                    "mixed:key".to_string(),
                    value.into_bytes(),
                    "text/plain".to_string(),
                )
                .await
        }));
    }

    // 等待所有读取操作完成
    for handle in read_handles {
        let _ = handle.await;
    }

    // 等待所有写入操作完成
    for handle in write_handles {
        let _ = handle.await;
    }

    // 验证键仍然存在
    assert!(cache.exists("mixed:key").await);
}

#[tokio::test]
async fn test_concurrent_different_keys() {
    let (cache, _temp_dir) = create_test_cache();
    let cache = Arc::new(cache);

    // 并发写入不同的键
    let mut handles = Vec::new();
    for i in 0..100 {
        let cache = cache.clone();
        let key = format!("key:{}", i);
        let value = format!("value{}", i);
        handles.push(tokio::spawn(async move {
            cache
                .set(key, value.into_bytes(), "text/plain".to_string())
                .await
        }));
    }

    // 等待所有写入完成
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    // 验证所有键都存在
    for i in 0..100 {
        let key = format!("key:{}", i);
        assert!(cache.exists(&key).await);
        let result = cache.get(&key).await.unwrap();
        assert_eq!(result, Some(format!("value{}", i).into_bytes()));
    }
}

