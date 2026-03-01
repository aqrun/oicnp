//! oic-cache 独立服务入口：Redis Protocol (6379) + gRPC (50051)。

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let _ = oic_cache::app::run().await;
    Ok(())
}
