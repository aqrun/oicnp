use axum::{
    Router,
    response::{IntoResponse, Html},
    routing::get,
    extract::{Extension, State},
};
use oic_core::AppContext;
use crate::models::HtmlTemplate;
use crate::views::render_home_index;
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::{ManifestChunk, ViteAssets};
use oic_cache::{Cache, CacheExt};

// 类型别名，帮助类型推导
type CacheExtension = Arc<Cache>;
type ManifestExtension = Arc<HashMap<String, ManifestChunk>>;

async fn index(
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let cache_key = "home:index";

    // 尝试从缓存获取 HTML
    if let Ok(Some(html)) = cache.get_html(cache_key).await {
        println!("缓存命中: {}", html);
        return Html(html).into_response();
    }

    // 缓存未命中，生成 HTML
    let html_template = match render_home_index(manifest.clone()).await {
        Ok(template) => template,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", e)
            ).into_response();
        }
    };

    // 将渲染后的 HTML 存入缓存
    let html_string = html_template.0.to_string();
    
    // 在开发模式下使用短 TTL，生产环境使用长 TTL
    #[cfg(debug_assertions)]
    let ttl_seconds = 1; // 开发模式：1 秒过期
    #[cfg(not(debug_assertions))]
    let ttl_seconds = 3600; // 生产环境：1 小时过期
    
    if let Err(e) = cache.set_html(
        cache_key.to_string(),
        html_string.as_str(),
        ttl_seconds
    ).await {
        eprintln!("Failed to cache HTML: {}", e);
    }

    html_template.into_response()
}

pub fn home_routes() -> Router<AppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
}