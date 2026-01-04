use oic_cache::{Cache, CacheConfig, CacheExt};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// cargo run --package oic_cache --example loco_compat
/// 
/// 演示 loco_rs 兼容的缓存 API
/// 
/// 这个示例展示了如何使用 CacheExt trait 提供的 loco_rs 兼容方法
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    println!("=== Loco_rs Compatible Cache API Demo ===\n");

    let cache = Cache::new(CacheConfig::default());

    // 1. Ping - 检查缓存是否可达
    println!("=== Step 1: Ping cache ===");
    cache.ping().await?;
    println!("✅ Cache is reachable\n");

    // 2. Insert - 插入序列化值
    println!("=== Step 2: Insert user ===");
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    cache.insert("user:1", &user).await?;
    println!("✅ Inserted user: {:?}\n", user);

    // 3. Contains key - 检查键是否存在
    println!("=== Step 3: Check if key exists ===");
    let exists = cache.contains_key("user:1").await?;
    println!("Key 'user:1' exists: {}\n", exists);

    // 4. Get - 获取并反序列化
    println!("=== Step 4: Get user ===");
    let retrieved: Option<User> = CacheExt::get(&cache, "user:1").await?;
    println!("Retrieved user: {:?}\n", retrieved);
    assert_eq!(retrieved, Some(user.clone()));

    // 5. Insert with expiry - 带过期时间的插入
    println!("=== Step 5: Insert with expiry ===");
    let user2 = User {
        id: 2,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    };
    cache.insert_with_expiry("user:2", &user2, Duration::from_secs(5)).await?;
    println!("✅ Inserted user2 with 5s TTL\n");

    // 6. Get or insert - 获取或插入
    println!("=== Step 6: Get or insert ===");
    let user3 = cache.get_or_insert("user:3", async {
        Ok(User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
        })
    }).await?;
    println!("✅ Got or inserted user3: {:?}\n", user3);

    // 再次调用应该从缓存获取
    let user3_cached = cache.get_or_insert("user:3", async {
        Ok(User {
            id: 999,
            name: "Should not be used".to_string(),
            email: "should@not.be".to_string(),
        })
    }).await?;
    println!("✅ Retrieved from cache (should be Charlie): {:?}\n", user3_cached);
    assert_eq!(user3_cached.id, 3);

    // 7. Get or insert with expiry - 带过期时间的获取或插入
    println!("=== Step 7: Get or insert with expiry ===");
    let user4 = cache.get_or_insert_with_expiry("user:4", Duration::from_secs(10), async {
        Ok(User {
            id: 4,
            name: "David".to_string(),
            email: "david@example.com".to_string(),
        })
    }).await?;
    println!("✅ Got or inserted user4 with 10s TTL: {:?}\n", user4);

    // 8. Remove - 删除键
    println!("=== Step 8: Remove key ===");
    cache.remove("user:1").await?;
    let exists_after_remove = cache.contains_key("user:1").await?;
    println!("Key 'user:1' exists after remove: {}\n", exists_after_remove);
    assert!(!exists_after_remove);

    // 9. Clear - 清空所有缓存
    println!("=== Step 9: Clear cache ===");
    cache.clear().await?;
    let all_keys_exist = cache.contains_key("user:2").await?
        || cache.contains_key("user:3").await?
        || cache.contains_key("user:4").await?;
    println!("Any keys exist after clear: {}\n", all_keys_exist);
    assert!(!all_keys_exist);

    println!("✅ All loco_rs compatible API tests passed!");
    println!("\n=== Summary ===");
    println!("✅ Core Cache only handles Vec<u8> (raw bytes)");
    println!("✅ CacheExt provides serialization/deserialization");
    println!("✅ All loco_rs API methods are available via trait");
    println!("✅ Clean separation of concerns");

    Ok(())
}

