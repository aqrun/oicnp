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
    
    // 当前目录是工作空间目录
    let current_dir = std::env::current_dir().expect("当前目录获取失败");
    let web_assets_dir = std::env::var("WEB_ASSETS_DIR")
        .expect("WEB_ASSETS_DIR 环境变量未设置");

    let app = Router::new()
        .merge(controllers::home_routes())
        .merge(controllers::blog_routes())
        // 静态资源路由配置
        .merge(static_assets_router(vite_serve))
        .nest_service("/public", ServeDir::new(
            current_dir.join(web_assets_dir.as_str()).to_string_lossy().to_string())
        )
        .with_state(app_ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9003").await.unwrap();
    println!("API 服务启动 {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
