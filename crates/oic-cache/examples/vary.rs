use oic_cache::{Cache, CacheConfig, VaryCondition, VaryValues};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Cache::new(CacheConfig::default());

    // 设置 Vary 缓存（根据语言和设备）
    cache
        .set_with_vary(
            "page:home".to_string(),
            b"<html lang='en'>Mobile Home</html>".to_vec(),
            "text/html".to_string(),
            vec![VaryCondition::AcceptLanguage, VaryCondition::UserAgent],
        )
        .await?;

    // 获取时需要提供 Vary 值
    let vary_values = VaryValues {
        language: Some("en".to_string()),
        encoding: None,
        user_agent: Some("mobile".to_string()),
        custom: HashMap::new(),
    };

    // 注意：set_with_vary 使用默认的 VaryValues，所以这里可能找不到
    // 这个示例展示了 API 的使用方式
    match cache.get_vary("page:home", &vary_values).await {
        Ok(Some(data)) => {
            println!("Got: {}", String::from_utf8_lossy(&data));
        }
        Ok(None) => {
            println!("Cache miss for vary key");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(())
}

