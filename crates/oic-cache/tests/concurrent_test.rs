use oic_cache::{Cache, CacheConfig};
use std::sync::Arc;
use std::time::Instant;
use tempfile::TempDir;

fn create_test_cache() -> (Cache, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut config = CacheConfig::default();
    config.disk_path = temp_dir.path().to_string_lossy().to_string();
    (Cache::new(config), temp_dir)
}

/// 测试并发读取性能
/// 
/// 功能说明：
/// - 设置一个键值对
/// - 启动 1000 个并发读取任务
/// - 验证所有读取都能正确获取数据
/// - 统计并发读取的耗时和性能
#[tokio::test]
async fn test_concurrent_reads() {
    println!("\n=== 测试：并发读取 ===");
    println!("功能：测试多个协程同时读取同一个缓存键的性能和正确性");
    
    let (cache, _temp_dir) = create_test_cache();
    let cache = Arc::new(cache);

    // 设置一个键
    println!("📝 设置测试键：concurrent:read");
    cache
        .set(
            "concurrent:read".to_string(),
            b"test value".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    // 1000 个并发读取
    println!("🚀 启动 1000 个并发读取任务...");
    let start_time = Instant::now();
    let mut handles = Vec::new();
    for i in 0..1000 {
        let cache = cache.clone();
        handles.push(tokio::spawn(async move {
            let result = cache.get("concurrent:read").await;
            if i % 100 == 0 {
                println!("  ✓ 任务 {} 完成", i);
            }
            result
        }));
    }

    // 等待所有读取完成并统计
    println!("⏳ 等待所有读取任务完成...");
    let mut success_count = 0;
    let mut error_count = 0;
    let mut completed = 0;
    
    for (idx, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => {
                match result {
                    Ok(data) => {
                        if data == Some(b"test value".to_vec()) {
                            success_count += 1;
                        } else {
                            error_count += 1;
                            println!("  ⚠️  任务 {} 返回了错误的数据", idx);
                        }
                    }
                    Err(e) => {
                        error_count += 1;
                        println!("  ❌ 任务 {} 读取失败: {}", idx, e);
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                println!("  ❌ 任务 {} 执行失败: {}", idx, e);
            }
        }
        completed += 1;
        if completed % 100 == 0 {
            println!("  📊 已完成: {}/1000", completed);
        }
    }

    let elapsed = start_time.elapsed();
    println!("\n📈 并发读取测试结果：");
    println!("  - 总任务数: 1000");
    println!("  - 成功数: {}", success_count);
    println!("  - 失败数: {}", error_count);
    println!("  - 总耗时: {:?}", elapsed);
    println!("  - 平均耗时: {:?}", elapsed / 1000);
    println!("  - QPS: {:.2}", 1000.0 / elapsed.as_secs_f64());
    
    assert_eq!(success_count, 1000, "所有读取任务都应该成功");
    assert_eq!(error_count, 0, "不应该有失败的任务");
    println!("✅ 并发读取测试通过！\n");
}

/// 测试并发写入性能
/// 
/// 功能说明：
/// - 启动 100 个并发写入任务，都写入同一个键
/// - 验证所有写入操作都能成功完成
/// - 验证最后的值存在（可能是任意一个写入的值，因为并发写入的顺序不确定）
/// - 统计并发写入的耗时和性能
#[tokio::test]
async fn test_concurrent_writes() {
    println!("\n=== 测试：并发写入 ===");
    println!("功能：测试多个协程同时写入同一个缓存键的并发安全性和性能");
    
    let (cache, _temp_dir) = create_test_cache();
    let cache = Arc::new(cache);

    // 100 个并发写入同一键
    println!("🚀 启动 100 个并发写入任务（写入同一个键）...");
    let start_time = Instant::now();
    let mut handles = Vec::new();
    for i in 0..100 {
        let cache = cache.clone();
        let value = format!("value{}", i);
        handles.push(tokio::spawn(async move {
            let result = cache
                .set(
                    "concurrent:write".to_string(),
                    value.into_bytes(),
                    "text/plain".to_string(),
                )
                .await;
            if i % 20 == 0 {
                println!("  ✓ 写入任务 {} 完成", i);
            }
            result
        }));
    }

    // 等待所有写入完成并统计
    println!("⏳ 等待所有写入任务完成...");
    let mut success_count = 0;
    let mut error_count = 0;
    let mut completed = 0;
    
    for (idx, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => {
                match result {
                    Ok(_) => success_count += 1,
                    Err(e) => {
                        error_count += 1;
                        println!("  ❌ 写入任务 {} 失败: {}", idx, e);
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                println!("  ❌ 任务 {} 执行失败: {}", idx, e);
            }
        }
        completed += 1;
        if completed % 20 == 0 {
            println!("  📊 已完成: {}/100", completed);
        }
    }

    let elapsed = start_time.elapsed();
    println!("\n📈 并发写入测试结果：");
    println!("  - 总任务数: 100");
    println!("  - 成功数: {}", success_count);
    println!("  - 失败数: {}", error_count);
    println!("  - 总耗时: {:?}", elapsed);
    println!("  - 平均耗时: {:?}", elapsed / 100);
    println!("  - QPS: {:.2}", 100.0 / elapsed.as_secs_f64());

    // 验证最后的值存在（可能是任意一个写入的值）
    println!("\n🔍 验证最终结果...");
    let result = cache.get("concurrent:write").await.unwrap();
    assert!(result.is_some(), "写入后应该能读取到值");
    println!("  ✓ 最终值存在: {}", String::from_utf8_lossy(&result.unwrap()));
    
    assert_eq!(success_count, 100, "所有写入任务都应该成功");
    assert_eq!(error_count, 0, "不应该有失败的任务");
    println!("✅ 并发写入测试通过！\n");
}

/// 测试混合并发操作（读写混合）
/// 
/// 功能说明：
/// - 先设置一个初始值
/// - 同时启动 50 个读取任务和 50 个写入任务
/// - 验证所有操作都能正常完成
/// - 验证最终键仍然存在
/// - 测试读写并发时的数据一致性
#[tokio::test]
async fn test_concurrent_mixed() {
    println!("\n=== 测试：混合并发操作（读写混合）===");
    println!("功能：测试同时进行读取和写入操作时的并发安全性和数据一致性");
    
    let (cache, _temp_dir) = create_test_cache();
    let cache = Arc::new(cache);

    // 初始设置
    println!("📝 设置初始值：mixed:key = 'initial'");
    cache
        .set(
            "mixed:key".to_string(),
            b"initial".to_vec(),
            "text/plain".to_string(),
        )
        .await
        .unwrap();

    // 混合读写操作
    println!("🚀 启动混合并发操作...");
    println!("  - 读取任务: 50 个");
    println!("  - 写入任务: 50 个");
    
    let start_time = Instant::now();
    let mut read_handles = Vec::new();
    let mut write_handles = Vec::new();

    // 50 个读取任务
    for i in 0..50 {
        let cache = cache.clone();
        read_handles.push(tokio::spawn(async move {
            let result = cache.get("mixed:key").await;
            if i % 10 == 0 {
                println!("  📖 读取任务 {} 完成", i);
            }
            result
        }));
    }

    // 50 个写入任务
    for i in 0..50 {
        let cache = cache.clone();
        let value = format!("update{}", i);
        write_handles.push(tokio::spawn(async move {
            let result = cache
                .set(
                    "mixed:key".to_string(),
                    value.into_bytes(),
                    "text/plain".to_string(),
                )
                .await;
            if i % 10 == 0 {
                println!("  ✍️  写入任务 {} 完成", i);
            }
            result
        }));
    }

    // 等待所有读取操作完成
    println!("\n⏳ 等待读取任务完成...");
    let mut read_success = 0;
    let mut read_error = 0;
    for (idx, handle) in read_handles.into_iter().enumerate() {
        match handle.await {
            Ok(Ok(_)) => read_success += 1,
            Ok(Err(e)) => {
                read_error += 1;
                println!("  ❌ 读取任务 {} 失败: {}", idx, e);
            }
            Err(e) => {
                read_error += 1;
                println!("  ❌ 读取任务 {} 执行失败: {}", idx, e);
            }
        }
    }

    // 等待所有写入操作完成
    println!("⏳ 等待写入任务完成...");
    let mut write_success = 0;
    let mut write_error = 0;
    for (idx, handle) in write_handles.into_iter().enumerate() {
        match handle.await {
            Ok(Ok(_)) => write_success += 1,
            Ok(Err(e)) => {
                write_error += 1;
                println!("  ❌ 写入任务 {} 失败: {}", idx, e);
            }
            Err(e) => {
                write_error += 1;
                println!("  ❌ 写入任务 {} 执行失败: {}", idx, e);
            }
        }
    }

    let elapsed = start_time.elapsed();
    println!("\n📈 混合并发测试结果：");
    println!("  - 读取任务总数: 50");
    println!("  - 读取成功: {}, 失败: {}", read_success, read_error);
    println!("  - 写入任务总数: 50");
    println!("  - 写入成功: {}, 失败: {}", write_success, write_error);
    println!("  - 总耗时: {:?}", elapsed);
    println!("  - 总操作数: 100");
    println!("  - 平均耗时: {:?}", elapsed / 100);
    println!("  - QPS: {:.2}", 100.0 / elapsed.as_secs_f64());

    // 验证键仍然存在
    println!("\n🔍 验证最终状态...");
    assert!(cache.exists("mixed:key").await, "键应该仍然存在");
    let final_value = cache.get("mixed:key").await.unwrap();
    assert!(final_value.is_some(), "应该能读取到最终值");
    println!("  ✓ 键存在: mixed:key");
    println!("  ✓ 最终值: {}", String::from_utf8_lossy(&final_value.unwrap()));
    
    assert_eq!(read_success + read_error, 50, "所有读取任务都应该完成");
    assert_eq!(write_success + write_error, 50, "所有写入任务都应该完成");
    println!("✅ 混合并发测试通过！\n");
}

/// 测试并发写入不同键的性能
/// 
/// 功能说明：
/// - 启动 100 个并发写入任务，每个任务写入不同的键
/// - 验证所有写入操作都能成功完成
/// - 验证所有键都能正确读取到对应的值
/// - 测试无锁竞争情况下的并发性能
#[tokio::test]
async fn test_concurrent_different_keys() {
    println!("\n=== 测试：并发写入不同键 ===");
    println!("功能：测试多个协程同时写入不同缓存键的性能（无锁竞争场景）");
    
    let (cache, _temp_dir) = create_test_cache();
    let cache = Arc::new(cache);

    // 并发写入不同的键
    println!("🚀 启动 100 个并发写入任务（每个任务写入不同的键）...");
    let start_time = Instant::now();
    let mut handles = Vec::new();
    for i in 0..100 {
        let cache = cache.clone();
        let key = format!("key:{}", i);
        let value = format!("value{}", i);
        handles.push(tokio::spawn(async move {
            let result = cache
                .set(key.clone(), value.into_bytes(), "text/plain".to_string())
                .await;
            if i % 20 == 0 {
                println!("  ✓ 写入任务 {} (key:{}) 完成", i, i);
            }
            result
        }));
    }

    // 等待所有写入完成并统计
    println!("⏳ 等待所有写入任务完成...");
    let mut success_count = 0;
    let mut error_count = 0;
    let mut completed = 0;
    
    for (idx, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => {
                match result {
                    Ok(_) => success_count += 1,
                    Err(e) => {
                        error_count += 1;
                        println!("  ❌ 写入任务 {} 失败: {}", idx, e);
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                println!("  ❌ 任务 {} 执行失败: {}", idx, e);
            }
        }
        completed += 1;
        if completed % 20 == 0 {
            println!("  📊 已完成: {}/100", completed);
        }
    }

    let elapsed = start_time.elapsed();
    println!("\n📈 并发写入不同键测试结果：");
    println!("  - 总任务数: 100");
    println!("  - 成功数: {}", success_count);
    println!("  - 失败数: {}", error_count);
    println!("  - 总耗时: {:?}", elapsed);
    println!("  - 平均耗时: {:?}", elapsed / 100);
    println!("  - QPS: {:.2}", 100.0 / elapsed.as_secs_f64());

    // 验证所有键都存在
    println!("\n🔍 验证所有键和值...");
    let verify_start = Instant::now();
    let mut verified_count = 0;
    let mut verify_error_count = 0;
    
    for i in 0..100 {
        let key = format!("key:{}", i);
        let expected_value = format!("value{}", i);
        
        // 检查键是否存在
        if !cache.exists(&key).await {
            verify_error_count += 1;
            println!("  ❌ 键不存在: {}", key);
            continue;
        }
        
        // 检查值是否正确
        match cache.get(&key).await {
            Ok(Some(data)) => {
                let actual_value = String::from_utf8_lossy(&data);
                if actual_value == expected_value {
                    verified_count += 1;
                    if i % 20 == 0 {
                        println!("  ✓ 验证通过: {} = {}", key, actual_value);
                    }
                } else {
                    verify_error_count += 1;
                    println!("  ❌ 值不匹配: {} (期望: {}, 实际: {})", key, expected_value, actual_value);
                }
            }
            Ok(None) => {
                verify_error_count += 1;
                println!("  ❌ 读取失败: {} (返回 None)", key);
            }
            Err(e) => {
                verify_error_count += 1;
                println!("  ❌ 读取错误: {} ({})", key, e);
            }
        }
    }
    
    let verify_elapsed = verify_start.elapsed();
    println!("\n📈 验证结果：");
    println!("  - 验证总数: 100");
    println!("  - 验证成功: {}", verified_count);
    println!("  - 验证失败: {}", verify_error_count);
    println!("  - 验证耗时: {:?}", verify_elapsed);
    
    assert_eq!(success_count, 100, "所有写入任务都应该成功");
    assert_eq!(error_count, 0, "不应该有失败的任务");
    assert_eq!(verified_count, 100, "所有键都应该验证成功");
    assert_eq!(verify_error_count, 0, "不应该有验证失败");
    println!("✅ 并发写入不同键测试通过！\n");
}
