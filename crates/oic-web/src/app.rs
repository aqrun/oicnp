use axum::Router;
use anyhow::Result;
use crate::controllers;
use tower_http::services::ServeDir;
use crate::{models::static_assets_router, context::init_context};

#[derive(vite_rs::Embed)]
#[root = "../../apps/web-app"]
struct Assets;

pub async fn run() -> Result<()> {
    let app_ctx = init_context().await?;

    #[cfg(debug_assertions)]
    let _guard = Assets::start_dev_server(true);

    // 等待 dev server 启动
    #[cfg(debug_assertions)]
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    let vite_serve = vite_rs_axum_0_8::ViteServe::new(Assets::boxed());
    // 静态资源目录
    let web_assets_dir = std::path::PathBuf::from(app_ctx.config.base_dir.as_str())
        .join(app_ctx.config.public_dir.as_str());
    let web_assets_dir = web_assets_dir.canonicalize().unwrap();
    println!("web_assets_dir: {:?}", web_assets_dir);
    // 监听地址
    let addr = format!("{}:{}", app_ctx.config.host.as_str(), app_ctx.config.port);

    let app = Router::new()
        .merge(controllers::home_routes())
        .merge(controllers::blog_routes())
        .merge(controllers::tool_routes())
        .merge(controllers::poetry_routes())
        .route("/public1/{*path}", axum::routing::get(debug_public_handler))
        .route_service("/public", ServeDir::new(&web_assets_dir))
        // 内联资源（Release 模式下处理嵌入的静态资源，如 /assets/*）
        // .merge(static_assets_router(vite_serve))
        .with_state(app_ctx);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Web 服务启动 {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// 添加一个调试处理器
async fn debug_public_handler(axum::extract::Path(path): axum::extract::Path<String>) -> impl axum::response::IntoResponse {
    println!("Requested path: /public/{}", path);
    (axum::http::StatusCode::OK, format!("Path: /public/{}", path))
}