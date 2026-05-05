use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::models::SiteConfig;
use oic_core::prelude::cache_client::{CacheDriver, RedisCache};
use std::path::PathBuf;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub public_dir: String,
    pub default_cache_seconds: u64,
    pub dev_cache_seconds: u64,
    pub api_url: String,
    pub site: SiteConfig,
    /// 当前工作目录
    pub base_dir: String,
    /// 处理器缓存时间
    pub handler_cache_time: i64,
    /// oic-cache Redis 地址，如 redis://127.0.0.1:6381
    pub redis_uri: String,
    pub db_uri: String,
}

/// Web 应用的上下文
#[derive(Clone)]
pub struct WebAppContext {
  pub config: WebConfig,
  pub cache: Arc<dyn CacheDriver>,
}

pub async fn load_config() -> Result<WebConfig> {
  let config_file = std::env::var("WEB_CONFIG_FILE")
    .unwrap_or_else(|_| "config/web.yaml".to_string());

  let mut cfg = WebConfig::default();

  if let Ok(content) = tokio::fs::read_to_string(config_file.as_str()).await {
    if let Ok(config) = serde_yaml::from_str::<WebConfig>(&content) {
      cfg = config;
    }
  }

  let default_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
  let base_dir = if let Ok(config_path) = std::fs::canonicalize(config_file.as_str()) {
    let mut c: PathBuf = config_path;
    c.push("../../");
    let a = std::fs::canonicalize(c).unwrap_or(default_dir);
    a
  } else { default_dir };
  cfg.base_dir = base_dir.to_string_lossy().to_string();

  Ok(cfg)
}

pub async fn init_context() -> Result<WebAppContext> {
  let config = load_config().await?;
  let db = init_db(config.db_uri.as_str()).await?;
  let cache = init_cache(db, config.redis_uri.as_str()).await?;

  Ok(WebAppContext {
    config,
    cache: Arc::new(cache) as Arc<dyn CacheDriver>,
  })
}

async fn init_cache(db: DatabaseConnection, redis_uri: &str) -> Result<RedisCache> {
  tracing::info!("redis cache uri: {}", redis_uri);

  let manager = RedisConnectionManager::new(redis_uri)?;
  let pool = Pool::builder()
    .max_size(16)
    .build(manager)
    .await
    .map_err(|e| anyhow::anyhow!("redis pool: {}", e))?;

  Ok(RedisCache::new(pool, db))
}

async fn init_db(db_uri: &str) -> Result<DatabaseConnection> {
  if db_uri.is_empty() {
      anyhow::bail!("db_uri is empty; set config/web.yaml db_uri");
  }
  tracing::info!("connecting database (admin)");

  let mut opt = ConnectOptions::new(db_uri.to_owned());
  opt.max_connections(10)
      .min_connections(1)
      .connect_timeout(std::time::Duration::from_secs(8))
      .sqlx_logging(true);

  let db = Database::connect(opt)
      .await
      .map_err(|e| anyhow::anyhow!("database connect: {}", e))?;

  Ok(db)
}