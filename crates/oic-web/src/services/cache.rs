use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use oic_cache::server::proto::cache_service_client::CacheServiceClient;
use oic_cache::server::proto::{GetRequest, SetRequest};
use tonic::transport::Channel;

/// Bytes 版缓存抽象：以二进制读写，便于零拷贝（如 HTML 片段）。
#[async_trait]
pub trait CacheDriver: Send + Sync {
    /// 按 key 取回缓存，未命中返回 `None`。
    async fn get_bytes(&self, key: &str) -> Result<Option<Bytes>>;
    /// 写入缓存并设置过期秒数。
    async fn set_ex_bytes(&self, key: &str, value: &[u8], ttl_secs: u64) -> Result<()>;
}

/// 基于 oic-cache gRPC 的缓存实现，走 CacheService Get/Set，无 Redis 协议握手问题。
#[derive(Clone)]
pub struct GrpcCache {
    client: CacheServiceClient<Channel>,
}

impl GrpcCache {
    pub fn new(client: CacheServiceClient<Channel>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl CacheDriver for GrpcCache {
    async fn get_bytes(&self, key: &str) -> Result<Option<Bytes>> {
        let req = GetRequest {
            key: key.to_string(),
        };
        let mut client = self.client.clone();
        let res = client
            .get(req)
            .await
            .map_err(|e| anyhow::anyhow!("cache grpc get: {}", e))?
            .into_inner();
        if res.found {
            Ok(Some(Bytes::from(res.data)))
        } else {
            Ok(None)
        }
    }

    async fn set_ex_bytes(&self, key: &str, value: &[u8], ttl_secs: u64) -> Result<()> {
        let req = SetRequest {
            key: key.to_string(),
            data: value.to_vec(),
            ttl_seconds: ttl_secs as i64,
        };
        let mut client = self.client.clone();
        client
            .set(req)
            .await
            .map_err(|e| anyhow::anyhow!("cache grpc set: {}", e))?;
        Ok(())
    }
}

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
/// 通过 `CacheDriver` 抽象，可对接 Redis 或其他实现。
///
/// # 参数
/// - `cache`: 实现 `CacheDriver` 的缓存（如 `RedisCache`）
/// - `cache_key`: 缓存键
/// - `render_fn`: 渲染函数，返回 `Result<Bytes>`
/// - `config`: 缓存配置（可选，默认使用开发/生产环境配置）
///
/// # 返回
/// - `Ok(Bytes)`: 成功（缓存命中或已渲染并缓存）
/// - `Err(e)`: 渲染或缓存失败
pub async fn get_cached_or_render<F, Fut>(
    cache: &dyn CacheDriver,
    cache_key: &str,
    render_fn: F,
    config: Option<CacheConfig>,
) -> Result<Bytes>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<Bytes, anyhow::Error>>,
{
    if let Some(data) = cache.get_bytes(cache_key).await? {
        return Ok(data);
    }

    let bytes: Bytes = render_fn().await?;

    let config = config.unwrap_or_default();
    let ttl_seconds = if cfg!(debug_assertions) {
        config.dev_ttl
    } else {
        config.prod_ttl
    };
    let ttl_u64 = ttl_seconds.max(0) as u64;

    cache.set_ex_bytes(cache_key, bytes.as_ref(), ttl_u64).await?;
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
            let _ = &$cache;
            let _ = crate::services::get_cached_or_render(
                $cache.as_ref(),
                "dev:none",
                move || async move { Ok(bytes::Bytes::from("")) },
                None,
            ).await;
            
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
            let render_future = $render;
            match crate::services::get_cached_or_render(
                $cache.as_ref(),
                $key,
                move || render_future,
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
            let _ = &$cache;
            let _ = crate::services::get_cached_or_render(
                $cache.as_ref(),
                "dev:none",
                move || async move { Ok(bytes::Bytes::from("")) },
                None,
            ).await;

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
            let render_future = $render;
            let config = crate::services::CacheConfig {
                dev_ttl: $ttl,
                prod_ttl: $ttl,
            };
            match crate::services::get_cached_or_render(
                $cache.as_ref(),
                $key,
                move || render_future,
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

