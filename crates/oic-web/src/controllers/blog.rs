use axum::{
    Router,
    routing::get,
    extract::{Extension, State, Path},
    response::{IntoResponse, Html},
    http::StatusCode,
};
use oic_core::AppContext;
use crate::views::{render_blog_list, render_blog_detail};
use crate::services::get_cached_or_render;
use askama::Template;
use oic_cache::Cache;
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::ManifestChunk;

// 类型别名，帮助类型推导
type CacheExtension = Arc<Cache>;
type ManifestExtension = Arc<HashMap<String, ManifestChunk>>;

/// 博客列表页
async fn blog_list(
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let manifest_clone = manifest.clone();
    match get_cached_or_render(
        &*cache,
        "blog:list",
        move || {
            let manifest = manifest_clone.clone();
            async move {
                let template = render_blog_list(None, manifest).await?;
                let html_string = template.0.render()
                    .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?;
                Ok(html_string.into_bytes())
            }
        },
        None,
    ).await {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Failed to render: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to render: {}", e)).into_response()
        }
    }
}

/// 分类博客列表页
async fn blog_list_by_category(
    Path(cat_vid): Path<String>,
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let cache_key = format!("blog:list:cat:{}", cat_vid);
    let cat_vid_clone = cat_vid.clone();
    let manifest_clone = manifest.clone();
    
    match get_cached_or_render(
        &*cache,
        &cache_key,
        move || {
            let cat_vid = cat_vid_clone.clone();
            let manifest = manifest_clone.clone();
            async move {
                let template = render_blog_list(Some(cat_vid), manifest).await?;
                let html_string = template.0.render()
                    .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?;
                Ok(html_string.into_bytes())
            }
        },
        None,
    ).await {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Failed to render: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to render: {}", e)).into_response()
        }
    }
}

/// 博客详情页
async fn blog_detail(
    Path(vid): Path<String>,
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let cache_key = format!("blog:detail:{}", vid);
    let vid_clone = vid.clone();
    let manifest_clone = manifest.clone();
    
    match get_cached_or_render(
        &*cache,
        &cache_key,
        move || {
            let vid = vid_clone.clone();
            let manifest = manifest_clone.clone();
            async move {
                let template = render_blog_detail(vid, manifest).await?;
                let html_string = template.0.render()
                    .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))?;
                Ok(html_string.into_bytes())
            }
        },
        None,
    ).await {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Failed to render: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to render: {}", e)).into_response()
        }
    }
}

pub fn blog_routes() -> Router<AppContext> {
    Router::new()
        .route("/blog/", get(blog_list))
        .route("/cat/:cat_vid/", get(blog_list_by_category))
        .route("/p/:vid/", get(blog_detail))
}

