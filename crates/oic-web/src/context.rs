use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use oic_cache::{Cache, CacheConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub public_dir: String,
    pub default_cache_seconds: u64,
    pub dev_cache_seconds: u64,
    pub api_url: String,
}

impl Default for WebConfig {
  fn default() -> Self {
    Self {
      host: "0.0.0.0".to_string(),
      port: 9003,
      public_dir: "public".to_string(),
      default_cache_seconds: 3600,
      dev_cache_seconds: 1,
      api_url: "http://localhost:5150".to_string(),
    }
  }
}

/// Web 应用的上下文
#[derive(Clone)]
pub struct WebAppContext {
  pub config: WebConfig,
  pub cache: Arc<Cache>,
}

pub async fn load_config() -> Result<WebConfig> {
  let config_file = std::env::var("WEB_CONFIG_FILE")
    .unwrap_or_else(|_| "config/web.yaml".to_string());

  let mut cfg = WebConfig::default();

  if let Ok(content) = tokio::fs::read_to_string(config_file).await {
    if let Ok(config) = serde_yaml::from_str::<WebConfig>(&content) {
      cfg = config;
    }
  }
  
  Ok(cfg)
}

pub async fn init_context() -> Result<WebAppContext> {
  let config = load_config().await?;
  let cache = init_cache(&config).await?;

  Ok(WebAppContext {
    config,
    cache: Arc::new(cache),
  })
}

async fn init_cache(cfg: &WebConfig) -> Result<Cache> {
  let mut config = CacheConfig::default();

  #[cfg(debug_assertions)]
  {
      config.storage.inline_threshold = 0;
      config.default_ttl_seconds = cfg.dev_cache_seconds as i64; // 1 秒过期
      // 在开发模式下，明确禁用 SWR，确保过期数据能被清理
      config.swr.enabled = false;
  }

  config.storage.auto_load_index = true; // 启用自动加载
  config.storage.auto_save_interval_seconds = 30; // 每 30 秒定期保存
  config.storage.auto_save_debounce_ms = 2000; // 更新后延迟 2 秒保存
  
  // 创建缓存并自动加载索引（如果存在）
  // load_index 会清理过期数据（如果 SWR 未启用）
  let cache = Cache::new_with_auto_load(config)
      .await
      .map_err(|e| anyhow::anyhow!("Failed to initialize cache: {}", e))?;

  let _ = cache.cleanup_expired().await;
  
  Ok(cache)
}