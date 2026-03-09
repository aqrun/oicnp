use crate::server::{start_grpc_server, RedisServer};
use crate::{Cache, CacheConfig};
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn load_config() -> CacheConfig {
    let config_file = std::env::var("CACHE_CONFIG_FILE")
        .unwrap_or_else(|_| "config/cache.yml".to_string());

    if let Ok(content) = tokio::fs::read_to_string(config_file.as_str()).await {
        if let Ok(cfg) = serde_yaml::from_str::<CacheConfig>(&content) {
            return cfg;
        } else {
            tracing::warn!(
                "Failed to parse cache config from {}, using default",
                config_file
            );
        }
    } else {
        tracing::warn!(
            "Cache config file {} not found, using default",
            config_file
        );
    }

    CacheConfig::default()
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = load_config().await;
    let redis_addr = String::from(config.server.redis_addr.as_str());
    let grpc_addr: SocketAddr = String::from(config.server.grpc_addr.as_str()).parse()?;

    let cache = Cache::new(config);
    if let Err(e) = cache.load_index().await {
        tracing::warn!("load_index failed: {} (continuing with empty index)", e);
    }
    let cache = Arc::new(cache);

    let redis_cache = cache.clone();
    tokio::spawn(async move {
        let server = RedisServer::new(redis_cache);
        if let Err(e) = server.run(&redis_addr).await {
            tracing::error!("Redis server error: {}", e);
        }
    });

    tracing::info!("gRPC server listening on {}", grpc_addr);
    start_grpc_server(cache, grpc_addr).await?;
    Ok(())
}