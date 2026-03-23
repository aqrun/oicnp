use anyhow::Result;
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::redis::AsyncCommands;
use bb8_redis::RedisConnectionManager;
use bytes::Bytes;

/// Bytes 版缓存抽象：以二进制读写，便于零拷贝（如 HTML 片段）。
#[async_trait]
pub trait CacheDriver: Send + Sync {
    /// 按 key 取回缓存，未命中返回 `None`。
    async fn get_bytes(&self, key: &str) -> Result<Option<Bytes>>;
    /// 写入缓存并设置过期秒数。
    async fn set_ex_bytes(&self, key: &str, value: &[u8], ttl_secs: u64) -> Result<()>;
}

/// 基于 bb8 + bb8-redis 的 Redis 缓存实现，使用 oic-cache 的 Redis 协议服务作为后端。
#[derive(Clone)]
pub struct RedisCache {
    pool: Pool<RedisConnectionManager>,
}

impl RedisCache {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CacheDriver for RedisCache {
    async fn get_bytes(&self, key: &str) -> Result<Option<Bytes>> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("redis pool get: {}", e))?;
        let cached: Option<Vec<u8>> = conn.get(key).await?;
        Ok(cached.map(Bytes::from))
    }

    async fn set_ex_bytes(&self, key: &str, value: &[u8], ttl_secs: u64) -> Result<()> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| anyhow::anyhow!("redis pool get: {}", e))?;
        conn.set_ex::<_, _, ()>(key, value, ttl_secs)
            .await
            .map_err(|e| anyhow::anyhow!("redis set_ex: {}", e))?;
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
