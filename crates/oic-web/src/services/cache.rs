use oic_cache::Cache;
use anyhow::Result;

/// 缓存配置
#[derive(Debug, Clone, Copy)]
pub struct CacheConfig {
    /// 开发模式的 TTL（秒）
    pub dev_ttl: i64,
    /// 生产环境的 TTL（秒）
    pub prod_ttl: i64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            dev_ttl: 1,
            prod_ttl: 3600,
        }
    }
}

/// 获取缓存或渲染新内容（统一实现）
/// 
/// 使用底层 `Vec<u8>` 数据类型进行缓存，但对外统一返回 `Result<String>`。
/// 渲染函数统一返回 `Result<Vec<u8>>`，在 controller 层处理模板渲染。
/// 函数内部自动处理 UTF-8 转换，简化 controller 代码。
/// 
/// # 参数
/// - `cache`: 缓存实例
/// - `cache_key`: 缓存键
/// - `render_fn`: 渲染函数，返回 `Result<Vec<u8>>`
/// - `config`: 缓存配置（可选，默认使用开发/生产环境配置）
/// 
/// # 返回
/// - `Ok(html_string)`: 成功（缓存命中或已渲染并缓存）
/// - `Err(e)`: 渲染、缓存或 UTF-8 转换失败
pub async fn get_cached_or_render<F, Fut>(
    cache: &Cache,
    cache_key: &str,
    render_fn: F,
    config: Option<CacheConfig>,
) -> Result<String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<Vec<u8>, anyhow::Error>>,
{
    // 先检查缓存（使用底层 get API）
    if let Ok(Some(bytes)) = cache.get(cache_key).await {
        // 将 Vec<u8> 转换为 String
        return String::from_utf8(bytes)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in cached data: {}", e));
    }

    // 缓存未命中，调用渲染函数
    let bytes = render_fn().await?;

    // 确定 TTL
    let config = config.unwrap_or_default();
    let ttl_seconds = if cfg!(debug_assertions) {
        config.dev_ttl
    } else {
        config.prod_ttl
    };

    // 将渲染后的数据存入缓存（使用底层 set_with_ttl API）
    if let Err(e) = cache.set_with_ttl(
        cache_key.to_string(),
        bytes.clone(),
        "text/html".to_string(),
        ttl_seconds
    ).await {
        eprintln!("Failed to cache: {}", e);
    }

    // 将 Vec<u8> 转换为 String
    String::from_utf8(bytes)
        .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in rendered data: {}", e))
}

