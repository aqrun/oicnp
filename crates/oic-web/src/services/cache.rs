use oic_cache::Cache;
use anyhow::Result;
use bytes::Bytes;

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
/// 使用 `Bytes` 数据类型进行缓存和返回，零拷贝，性能最优。
/// 
/// # 参数
/// - `cache`: 缓存实例
/// - `cache_key`: 缓存键
/// - `render_fn`: 渲染函数，返回 `Result<Bytes>`
/// - `config`: 缓存配置（可选，默认使用开发/生产环境配置）
/// 
/// # 返回
/// - `Ok(Bytes)`: 成功（缓存命中或已渲染并缓存）
/// - `Err(e)`: 渲染或缓存失败
pub async fn get_cached_or_render<F, Fut>(
    cache: &Cache,
    cache_key: &str,
    render_fn: F,
    config: Option<CacheConfig>,
) -> Result<Bytes>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<Bytes, anyhow::Error>>,
{
    // 先检查缓存（使用底层 get API，返回 Bytes）
    if let Ok(Some(bytes)) = cache.get(cache_key).await {
        // ✅ 直接返回 Bytes，零拷贝
        return Ok(bytes);
    }

    // 缓存未命中，调用渲染函数
    let bytes: Bytes = render_fn().await?;

    // 确定 TTL
    let config = config.unwrap_or_default();
    let ttl_seconds = if cfg!(debug_assertions) {
        config.dev_ttl
    } else {
        config.prod_ttl
    };

    // 将渲染后的数据存入缓存（使用底层 set_with_ttl API，接受 Bytes）
    // bytes.clone() 现在是零拷贝的（引用计数）
    if let Err(e) = cache.set_with_ttl(
        cache_key.to_string(),
        bytes.clone(),
        "text/html".to_string(),
        ttl_seconds
    ).await {
        eprintln!("Failed to cache: {}", e);
    }

    // ✅ 直接返回 Bytes，零拷贝
    Ok(bytes)
}

/// 简化缓存调用的宏
/// 
/// 该宏简化 `get_cached_or_render` 的调用，减少代码冗余。
/// 宏直接返回 `Response`，包含错误处理逻辑。
/// 
/// **开发模式行为**：在开发模式下不使用缓存，直接渲染以便快速发现错误。
/// **生产模式行为**：使用缓存提高性能。
/// 
/// # 使用示例
/// 
/// ```rust
/// // 基础用法
/// async fn index(...) -> impl IntoResponse {
///     cached!(&*cache, "home:index", render_home_index(manifest.clone()))
/// }
/// 
/// // 自定义 TTL
/// async fn blog_list(...) -> impl IntoResponse {
///     cached!(&*cache, "blog:list", render_blog_list(None, manifest.clone()), 7200)
/// }
/// ```
#[macro_export]
macro_rules! cached {
    ($cache:expr, $key:expr, $render:expr) => {{
        // 开发模式：不使用缓存，直接渲染以便快速发现错误
        #[cfg(debug_assertions)]
        {
            match $render.await {
                Ok(bytes) => axum::response::Html(bytes).into_response(),
                Err(e) => {
                    // 使用 tracing 记录错误
                    tracing::error!(key = $key, error = %e, "Render failed in development mode");
                    
                    // 输出详细的错误信息到控制台
                    eprintln!("\n❌ RENDER ERROR [{}]", $key);
                    eprintln!("   Error: {}", e);
                    // 显示错误链
                    for cause in e.chain().skip(1) {
                        eprintln!("   Caused by: {}", cause);
                    }
                    // 显示完整的调试信息（包括 backtrace，如果可用）
                    eprintln!("   Debug info:\n{:?}", e);
                    eprintln!();
                    
                    crate::views::handle_error(
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        e
                    ).await
                }
            }
        }
        
        // 生产模式：使用缓存
        #[cfg(not(debug_assertions))]
        {
            match crate::services::get_cached_or_render(
                $cache,
                $key,
                move || async move { $render.await },
                None,
            ).await {
                Ok(bytes) => axum::response::Html(bytes).into_response(),
                Err(e) => crate::views::handle_error(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    e
                ).await,
            }
        }
    }};
    
    // 支持自定义 TTL
    ($cache:expr, $key:expr, $render:expr, $ttl:expr) => {{
        // 开发模式：不使用缓存，直接渲染以便快速发现错误
        #[cfg(debug_assertions)]
        {
            match $render.await {
                Ok(bytes) => axum::response::Html(bytes).into_response(),
                Err(e) => {
                    // 使用 tracing 记录错误
                    tracing::error!(key = $key, ttl = $ttl, error = %e, "Render failed in development mode");
                    
                    // 输出详细的错误信息到控制台
                    eprintln!("\n❌ RENDER ERROR [{}] (TTL: {})", $key, $ttl);
                    eprintln!("   Error: {}", e);
                    // 显示错误链
                    for cause in e.chain().skip(1) {
                        eprintln!("   Caused by: {}", cause);
                    }
                    // 显示完整的调试信息（包括 backtrace，如果可用）
                    eprintln!("   Debug info:\n{:?}", e);
                    eprintln!();
                    
                    crate::views::handle_error(
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        e
                    ).await
                }
            }
        }
        
        // 生产模式：使用缓存
        #[cfg(not(debug_assertions))]
        {
            let config = crate::services::CacheConfig {
                dev_ttl: $ttl,
                prod_ttl: $ttl,
            };
            match crate::services::get_cached_or_render(
                $cache,
                $key,
                move || async move { $render.await },
                Some(config),
            ).await {
                Ok(bytes) => axum::response::Html(bytes).into_response(),
                Err(e) => crate::views::handle_error(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    e
                ).await,
            }
        }
    }};
}

