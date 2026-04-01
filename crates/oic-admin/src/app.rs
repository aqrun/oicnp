use axum::{Router, routing::get};
use anyhow::Result;
use crate::controllers;
use tower_http::services::ServeDir;
use crate::context::init_context;

pub async fn run() -> Result<()> {
    let app_ctx = init_context().await?;

    // 监听地址
    let addr = format!("{}:{}", app_ctx.config.host.as_str(), app_ctx.config.admin.port);
    let app = Router::new()
        .merge(controllers::home_routes())
        .nest_service("/public", ServeDir::new(app_ctx.config.admin.public_dir.as_str()))
        .fallback(get(controllers::home_index))
        .with_state(app_ctx);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Admin 服务启动 {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
