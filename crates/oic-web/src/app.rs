use axum::{Router, Extension};
use anyhow::Result;
use crate::controllers;
use oic_core::app::{create_context, get_environment};
use tower_http::services::ServeDir;
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::ManifestChunk;

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

    let app = Router::new()
        .merge(controllers::home_routes())
        .nest_service("/assets", ServeDir::new(
            current_dir.join(web_assets_dir.as_str()).to_string_lossy().to_string())
        )
        .layer(Extension(manifest_state))
        .with_state(app_ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9003").await.unwrap();
    println!("API 服务启动 {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
