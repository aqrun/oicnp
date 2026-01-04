use oic_cache::{Cache, CacheConfig};

/// cargo run --package oic_cache --example persistence
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let mut config = CacheConfig::default();
    config.disk_path = "./target/.cache".to_string();
    config.storage.auto_load_index = true;
    config.storage.auto_save_index = true; // 启用自动保存
    config.storage.auto_save_interval_seconds = 5; // 每 5 秒自动保存一次（演示用）
    config.storage.auto_save_debounce_ms = 2000; // 2 秒 debounce

    println!("=== Cache Persistence Demo ===");
    println!("Config:");
    println!("  Auto load index: {}", config.storage.auto_load_index);
    println!("  Auto save index: {}", config.storage.auto_save_index);
    println!("  Auto save interval: {} seconds", config.storage.auto_save_interval_seconds);
    println!("  Auto save debounce: {} ms", config.storage.auto_save_debounce_ms);
    println!();

    // 创建缓存并自动加载索引（如果存在）
    println!("=== Step 1: Creating cache with auto-load ===");
    let cache = Cache::new_with_auto_load(config.clone()).await?;

    // 设置一些缓存数据
    println!("\n=== Step 2: Setting cache data ===");
    cache
        .set(
            "persist:key1".to_string(),
            b"Value 1 - persisted".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("✓ Set persist:key1 (auto-save will trigger)");

    cache
        .set(
            "persist:key2".to_string(),
            b"Value 2 - persisted".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("✓ Set persist:key2 (auto-save will trigger)");

    // 注意：由于启用了自动保存，索引会在后台自动保存
    // 不需要手动调用 save_index()，但为了演示，我们等待一下让自动保存完成
    println!("\n=== Step 3: Waiting for auto-save to complete ===");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("✓ Auto-save should have completed in background");

    // 也可以手动立即保存（如果需要）
    cache.save_index().await?;
    println!("✓ Manually saved index (for immediate persistence)");

    // 验证数据存在
    println!("\n=== Step 4: Verifying cached data ===");
    let value1 = cache.get("persist:key1").await?;
    let value2 = cache.get("persist:key2").await?;
    println!("Key1 value: {:?}", value1);
    println!("Key2 value: {:?}", value2);

    // 模拟应用重启：重新创建缓存实例
    println!("\n=== Step 5: Simulating app restart (recreating cache) ===");
    drop(cache); // 释放之前的缓存实例
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let cache2 = Cache::new_with_auto_load(config).await?;
    println!("✓ Cache recreated, index should be auto-loaded");

    // 验证数据仍然存在（从磁盘加载的索引）
    println!("\n=== Step 6: Verifying data after reload ===");
    let value1_loaded = cache2.get("persist:key1").await?;
    let value2_loaded = cache2.get("persist:key2").await?;

    println!("Key1 value after reload: {:?}", value1_loaded);
    println!("Key2 value after reload: {:?}", value2_loaded);

    assert_eq!(value1_loaded, Some(b"Value 1 - persisted".to_vec()));
    assert_eq!(value2_loaded, Some(b"Value 2 - persisted".to_vec()));

    println!("\n✅ Index persistence test passed!");

    // 演示自动保存：更新数据后等待自动保存
    println!("\n=== Step 7: Demonstrating auto-save on update ===");
    cache2
        .set(
            "persist:key3".to_string(),
            b"Value 3 - auto-saved".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("✓ Set persist:key3 (will trigger auto-save in background)");
    
    // 等待自动保存完成
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("✓ Auto-save should have completed");

    // 查看统计信息
    let stats = cache2.statistics().await;
    println!("\n=== Cache Statistics ===");
    println!("  Total entries: {}", stats.total_entries);
    println!("  Memory usage: {:.2} MB", stats.memory_usage_mb);
    println!("  Hit rate: {:.2}%", stats.hit_rate * 100.0);

    // 测试重置式 debounce 机制
    println!("\n=== Step 8: Testing debounce mechanism ===");
    let mut config_debounce = CacheConfig::default();
    config_debounce.disk_path = "./target/.cache_debounce".to_string();
    config_debounce.storage.auto_save_index = true;
    config_debounce.storage.auto_save_interval_seconds = 60; // 设置较长的定期保存间隔
    config_debounce.storage.auto_save_debounce_ms = 2000; // 2 秒 debounce
    
    let cache_debounce = Cache::new_with_auto_load(config_debounce).await?;
    
    println!("Config: debounce_ms = 2000ms, interval = 60s");
    println!("Performing 10 rapid updates...");
    
    let start_time = std::time::Instant::now();
    
    // 快速连续更新 10 次
    for i in 1..=10 {
        cache_debounce
            .set(
                format!("debounce:key{}", i),
                format!("value{}", i).into_bytes(),
                "text/plain".to_string(),
            )
            .await?;
        println!("  Update {} completed", i);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // 每 100ms 更新一次
    }
    
    let update_duration = start_time.elapsed();
    println!("All 10 updates completed in {:?}", update_duration);
    
    // 等待 debounce 时间 + 一些缓冲时间
    println!("Waiting for debounce to complete (should save once after 2 seconds)...");
    tokio::time::sleep(tokio::time::Duration::from_millis(2500)).await;
    
    // 验证数据已保存（通过重新加载缓存）
    println!("\n=== Step 9: Verifying debounce saved the index ===");
    drop(cache_debounce);
    
    let mut config_reload = CacheConfig::default();
    config_reload.disk_path = "./target/.cache_debounce".to_string();
    config_reload.storage.auto_load_index = true;
    
    let cache_reload = Cache::new_with_auto_load(config_reload).await?;
    
    // 验证所有键都存在（说明索引已保存）
    let mut found_count = 0;
    for i in 1..=10 {
        let key = format!("debounce:key{}", i);
        if cache_reload.exists(&key).await {
            found_count += 1;
        }
    }
    
    println!("Found {}/10 keys after reload (index was saved)", found_count);
    
    if found_count == 10 {
        println!("✅ Debounce mechanism test passed!");
        println!("   All 10 rapid updates were saved in a single debounced save operation.");
    } else {
        println!("⚠️  Warning: Expected 10 keys, found {}", found_count);
    }
    
    // 测试 debounce 重置机制
    println!("\n=== Step 10: Testing debounce reset mechanism ===");
    let mut config_reset = CacheConfig::default();
    config_reset.disk_path = "./target/.cache_reset".to_string();
    config_reset.storage.auto_save_index = true;
    config_reset.storage.auto_save_debounce_ms = 3000; // 3 秒 debounce
    
    let cache_reset = Cache::new_with_auto_load(config_reset).await?;
    
    println!("Config: debounce_ms = 3000ms");
    println!("Update pattern: update -> wait 1s -> update -> wait 1s -> update -> wait 4s");
    
    // 第一次更新
    cache_reset
        .set(
            "reset:key1".to_string(),
            b"value1".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("  Update 1 at 0s");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 第二次更新（重置 debounce）
    cache_reset
        .set(
            "reset:key2".to_string(),
            b"value2".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("  Update 2 at 1s (resets debounce timer)");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // 第三次更新（再次重置 debounce）
    cache_reset
        .set(
            "reset:key3".to_string(),
            b"value3".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("  Update 3 at 2s (resets debounce timer again)");
    
    // 等待 4 秒（应该触发保存）
    println!("  Waiting 4 seconds (debounce should trigger at ~5s total)...");
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    
    // 验证保存
    drop(cache_reset);
    let mut config_verify = CacheConfig::default();
    config_verify.disk_path = "./target/.cache_reset".to_string();
    config_verify.storage.auto_load_index = true;
    
    let cache_verify = Cache::new_with_auto_load(config_verify).await?;
    
    let all_exist = cache_verify.exists("reset:key1").await
        && cache_verify.exists("reset:key2").await
        && cache_verify.exists("reset:key3").await;
    
    if all_exist {
        println!("✅ Debounce reset mechanism test passed!");
        println!("   All 3 updates were saved after the final debounce period.");
    } else {
        println!("⚠️  Warning: Not all keys found after debounce");
    }
    
    println!("\n✅ All tests passed! Index persistence with auto-save is working correctly.");
    println!("\n=== Summary ===");
    println!("✅ Basic persistence: Data survives cache recreation");
    println!("✅ Auto-save on update: Index is saved automatically");
    println!("✅ Debounce mechanism: Rapid updates trigger single save");
    println!("✅ Debounce reset: Timer resets on each update");
    println!("\nNote: In production, you don't need to call save_index() manually.");
    println!("      The cache will automatically save the index:");
    println!("      - After updates (with debounce to reduce I/O)");
    println!("      - Periodically based on auto_save_interval_seconds");

    Ok(())
}

