use oic_cache::{Cache, CacheConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建缓存实例
    let cache = Cache::new(CacheConfig::default());
    
    // 2. 设置缓存
    cache.set(
        "user:123".to_string(),
        b"Alice".to_vec(),
        "text/plain".to_string(),
    ).await?;
    
    // 3. 读取缓存
    if let Some(data) = cache.get("user:123").await? {
        println!("User: {}", String::from_utf8_lossy(&data));
    }
    
    // 4. 失效缓存
    cache.invalidate("user:123").await?;
    
    // 5. 查看统计
    let stats = cache.statistics().await;
    println!("Stats: {:?}", stats);
    println!("Hit rate: {:.2}%", stats.hit_rate * 100.0);
    
    Ok(())
}

