use axum::{Router, Extension};
use anyhow::Result;
use crate::controllers;
use oic_core::app::{create_context, get_environment};
use tower_http::services::ServeDir;
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::ManifestChunk;
use oic_cache::{Cache, CacheConfig};

pub async fn run() -> Result<()> {
    let environment = get_environment();
    let app_ctx = create_context(&environment).await?;

    // 当前目录是工作空间目录
    let current_dir = std::env::current_dir().expect("当前目录获取失败");
    let web_assets_dir = std::env::var("WEB_ASSETS_DIR")
        .expect("WEB_ASSETS_DIR 环境变量未设置");
    let manifest_file = current_dir
        .join(web_assets_dir.as_str())
        .join(".vite/manifest.json")
        .to_string_lossy()
        .to_string();
    let manifest = ManifestChunk::from_path(String::from(manifest_file));
    let manifest_state: Arc<HashMap<String, ManifestChunk>> = Arc::new(manifest);

    // 缓存系统
    let cache = init_cache().await.expect("缓存系统初始化失败");
    let cache_state = Arc::new(cache);

    let app = Router::new()
        .merge(controllers::home_routes())
        .merge(controllers::blog_routes())
        .nest_service("/assets", ServeDir::new(
            current_dir.join(web_assets_dir.as_str()).to_string_lossy().to_string())
        )
        .layer(Extension(manifest_state))
        .layer(Extension(cache_state))
        .with_state(app_ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9003").await.unwrap();
    println!("API 服务启动 {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn init_cache() -> Result<Cache> {
    let mut config = CacheConfig::default();

    #[cfg(debug_assertions)]
    {
        config.storage.inline_threshold = 0;
        config.default_ttl_seconds = 1; // 1 秒过期
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
