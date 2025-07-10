use std::time::Duration;
use bb8::Pool;
use bb8_redis::{
    bb8,
    RedisConnectionManager,
    redis::{cmd, AsyncCommands},
};
use anyhow::Result;
use loco_rs::app::AppContext;

pub async fn get_redis_pool(uri: &str, max_size: u32) -> Result<Pool<RedisConnectionManager>> {
    let manager = RedisConnectionManager::new(uri)?;
    let pool = Pool::builder()
        .max_size(max_size)
        .build(manager)
        .await?;
    Ok(pool)
}

pub type RedisPool = std::sync::Arc<Pool<RedisConnectionManager>>;

pub struct Redis {
    pool: RedisPool,
}

impl Redis {
    pub fn new(pool: RedisPool) -> Self {
        Self {
            pool,
        }
    }

    pub async fn from(ctx: &AppContext) -> Result<Self> {
        let pool = ctx.shared_store.get::<RedisPool>()
            .ok_or(anyhow::anyhow!("Redis pool not found"))?;

        Ok(Self {
            pool,
        })
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.pool.get().await?;
        
        let r: Option<String> = conn.get(key).await?;
        Ok(r)
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;
        conn.set::<_, _, ()>(key, value).await?;
        Ok(())
    }

    pub async fn remove(&self, key: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;
        conn.del::<_, ()>(key).await?;
        Ok(())
    }

    pub async fn set_ex(
        &self,
        key: &str,
        value: &str,
        duration: Duration,
    ) -> Result<()> {
        let mut conn = self.pool.get().await?;
        // Redis expects the expiry in seconds as a u64
        conn.set_ex::<_, _, ()>(key, value, duration.as_secs())
            .await?;
        Ok(())
    }

}
