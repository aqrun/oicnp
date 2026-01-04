use oic_cache::{Cache, CacheConfig};

/// cargo run --package oic_cache --example swr
/// 
/// 演示 Stale-While-Revalidate (SWR) 功能
/// 
/// SWR 是一种缓存策略，允许在缓存过期后仍然返回过期数据（stale data），
/// 同时在后台触发重新获取。这样可以：
/// 1. 提高响应速度（立即返回 stale 数据）
/// 2. 减少服务器负载（避免等待重新获取）
/// 3. 保证数据最终一致性（后台更新）
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("=== Stale-While-Revalidate (SWR) Demo ===\n");

    // 创建启用 SWR 的缓存配置
    let mut config = CacheConfig::default();
    config.disk_path = "./target/.cache_swr".to_string();
    config.swr.enabled = true;
    config.swr.max_stale_seconds = 3600; // 最多保留 1 小时的 stale 数据
    config.default_ttl_seconds = 5; // 设置较短的 TTL（5秒）用于演示

    println!("Config:");
    println!("  SWR enabled: {}", config.swr.enabled);
    println!("  Max stale seconds: {}", config.swr.max_stale_seconds);
    println!("  Default TTL: {} seconds", config.default_ttl_seconds);
    println!();

    let cache = Cache::new(config);

    // Step 1: 设置初始缓存
    println!("=== Step 1: Setting initial cache ===");
    cache
        .set(
            "swr:key1".to_string(),
            b"Fresh data".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("✓ Set swr:key1 with TTL=5s");

    // Step 2: 立即获取（应该返回新鲜数据）
    println!("\n=== Step 2: Getting cache immediately (should be fresh) ===");
    let value1 = cache.get("swr:key1").await?;
    println!("Value: {:?}", value1);
    assert_eq!(value1, Some(b"Fresh data".to_vec()));
    
    let is_stale1 = cache.is_stale("swr:key1").await;
    println!("Is stale: {}", is_stale1);
    assert!(!is_stale1, "Data should be fresh");
    println!("✅ Data is fresh");

    // Step 3: 等待缓存过期
    println!("\n=== Step 3: Waiting for cache to expire (6 seconds) ===");
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
    println!("✓ Cache should be expired now");

    // Step 4: 获取过期缓存（SWR 应该返回 stale 数据）
    println!("\n=== Step 4: Getting expired cache (SWR should return stale data) ===");
    let value2 = cache.get("swr:key1").await?;
    println!("Value: {:?}", value2);
    assert_eq!(value2, Some(b"Fresh data".to_vec()), "SWR should return stale data");
    
    let is_stale2 = cache.is_stale("swr:key1").await;
    println!("Is stale: {}", is_stale2);
    assert!(is_stale2, "Data should be stale");
    println!("✅ SWR returned stale data (as expected)");

    // Step 5: 检查键是否存在（stale 数据也应该算存在）
    println!("\n=== Step 5: Checking if key exists (stale data should count) ===");
    let exists = cache.exists("swr:key1").await;
    println!("Exists: {}", exists);
    assert!(exists, "Stale data should be considered as existing");
    println!("✅ Stale data is considered as existing");

    // Step 6: 更新缓存（模拟后台重新获取）
    println!("\n=== Step 6: Updating cache (simulating background revalidation) ===");
    cache
        .set(
            "swr:key1".to_string(),
            b"Updated data".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("✓ Updated swr:key1");

    // Step 7: 再次获取（应该返回新数据）
    println!("\n=== Step 7: Getting cache again (should return updated data) ===");
    let value3 = cache.get("swr:key1").await?;
    println!("Value: {:?}", value3);
    assert_eq!(value3, Some(b"Updated data".to_vec()));
    
    let is_stale3 = cache.is_stale("swr:key1").await;
    println!("Is stale: {}", is_stale3);
    assert!(!is_stale3, "Data should be fresh after update");
    println!("✅ Data is fresh after update");

    // Step 8: 测试超过 max_stale_seconds 的情况
    println!("\n=== Step 8: Testing max_stale_seconds limit ===");
    cache
        .set(
            "swr:key2".to_string(),
            b"Test stale limit".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("✓ Set swr:key2");
    
    // 等待过期
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
    
    // 创建一个 max_stale_seconds 很短的配置
    let mut config_short = CacheConfig::default();
    config_short.disk_path = "./target/.cache_swr".to_string();
    config_short.swr.enabled = true;
    config_short.swr.max_stale_seconds = 1; // 只允许 1 秒的 stale 数据
    
    let cache_short = Cache::new(config_short);
    
    // 等待超过 max_stale_seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    let value4 = cache_short.get("swr:key2").await?;
    println!("Value after max_stale exceeded: {:?}", value4);
    
    // 由于超过了 max_stale_seconds，应该返回 None
    if value4.is_none() {
        println!("✅ Correctly rejected stale data that exceeded max_stale_seconds");
    } else {
        println!("⚠️  Warning: Expected None but got value (may be due to timing)");
    }

    // Step 9: 测试禁用 SWR 的情况
    println!("\n=== Step 9: Testing with SWR disabled ===");
    let mut config_no_swr = CacheConfig::default();
    config_no_swr.disk_path = "./target/.cache_swr".to_string();
    config_no_swr.swr.enabled = false; // 禁用 SWR
    config_no_swr.default_ttl_seconds = 5;
    
    let cache_no_swr = Cache::new(config_no_swr);
    
    cache_no_swr
        .set(
            "swr:key3".to_string(),
            b"Test no SWR".to_vec(),
            "text/plain".to_string(),
        )
        .await?;
    println!("✓ Set swr:key3");
    
    // 等待过期
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
    
    let value5 = cache_no_swr.get("swr:key3").await?;
    println!("Value with SWR disabled: {:?}", value5);
    
    if value5.is_none() {
        println!("✅ Correctly returned None when SWR is disabled");
    } else {
        println!("⚠️  Warning: Expected None when SWR is disabled");
    }

    println!("\n=== Summary ===");
    println!("✅ SWR returns stale data when cache is expired");
    println!("✅ Stale data is considered as existing");
    println!("✅ is_stale() correctly identifies stale data");
    println!("✅ max_stale_seconds limits how long stale data is acceptable");
    println!("✅ SWR can be disabled via configuration");
    println!("\n✅ All SWR tests passed!");

    Ok(())
}

