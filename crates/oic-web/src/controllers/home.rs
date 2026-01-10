use axum::{
    Router,
    routing::get,
    extract::{Extension, State},
    response::{IntoResponse, Html},
    http::StatusCode,
};
use oic_core::AppContext;
use crate::views::render_home_index;
use crate::services::get_cached_or_render;
use askama::Template;
use oic_cache::Cache;
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::ManifestChunk;

// 类型别名，帮助类型推导
type CacheExtension = Arc<Cache>;
type ManifestExtension = Arc<HashMap<String, ManifestChunk>>;

async fn index(
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let manifest_clone = manifest.clone();
    
    match get_cached_or_render(
        &*cache,
        "home:index",
        move || {
            let manifest = manifest_clone.clone();
            async move {
                // 在 controller 层处理模板渲染，转换为 Vec<u8>
                let template = render_home_index(manifest).await?;
                let html_string = template.0.render()
                    .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?;
                Ok(html_string.into_bytes())
            }
        },
        None,
    ).await {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to render: {}", e)).into_response()
        }
    }
}

pub fn home_routes() -> Router<AppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
}