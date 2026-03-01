use crate::server::{start_grpc_server, RedisServer};
use crate::{Cache, CacheConfig};
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = CacheConfig::default();
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