use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use oic_cache::{Cache, CacheConfig};
use crate::models::SiteConfig;
use std::path::PathBuf;

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