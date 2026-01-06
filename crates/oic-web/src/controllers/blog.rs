use axum::{
    Router,
    response::{IntoResponse, Html},
    routing::get,
    extract::{Extension, State, Path},
};
use oic_core::AppContext;
use crate::views::{render_blog_list, render_blog_detail};
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::ManifestChunk;
use oic_cache::{Cache, CacheExt};

// 类型别名，帮助类型推导
type CacheExtension = Arc<Cache>;
type ManifestExtension = Arc<HashMap<String, ManifestChunk>>;

/// 博客列表页
async fn blog_list(
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let cache_key = "blog:list";

    // 尝试从缓存获取 HTML
    if let Ok(Some(html)) = cache.get_html(cache_key).await {
        return Html(html).into_response();
    }

    // 缓存未命中，生成 HTML
    let html_template = match render_blog_list(None, manifest.clone()).await {
        Ok(template) => template,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", e)
            ).into_response();
        }
    };

    // 将渲染后的 HTML 存入缓存
    // 先渲染模板获取 HTML 字符串用于缓存
    let html_string = match askama::Template::render(&html_template.0) {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            return html_template.into_response();
        }
    };
    
    #[cfg(debug_assertions)]
    let ttl_seconds = 1;
    #[cfg(not(debug_assertions))]
    let ttl_seconds = 3600;
    
    if let Err(e) = cache.set_html(
        cache_key.to_string(),
        html_string.as_str(),
        ttl_seconds
    ).await {
        eprintln!("Failed to cache HTML: {}", e);
    }

    html_template.into_response()
}

/// 分类博客列表页
async fn blog_list_by_category(
    Path(cat_vid): Path<String>,
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let cache_key = format!("blog:list:cat:{}", cat_vid);

    // 尝试从缓存获取 HTML
    if let Ok(Some(html)) = cache.get_html(&cache_key).await {
        return Html(html).into_response();
    }

    // 缓存未命中，生成 HTML
    let html_template = match render_blog_list(Some(cat_vid.clone()), manifest.clone()).await {
        Ok(template) => template,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", e)
            ).into_response();
        }
    };

    // 将渲染后的 HTML 存入缓存
    let html_string = html_template.0.render().unwrap_or_default();
    
    #[cfg(debug_assertions)]
    let ttl_seconds = 1;
    #[cfg(not(debug_assertions))]
    let ttl_seconds = 3600;
    
    if let Err(e) = cache.set_html(
        cache_key,
        html_string.as_str(),
        ttl_seconds
    ).await {
        eprintln!("Failed to cache HTML: {}", e);
    }

    html_template.into_response()
}

/// 博客详情页
async fn blog_detail(
    Path(vid): Path<String>,
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let cache_key = format!("blog:detail:{}", vid);

    // 尝试从缓存获取 HTML
    if let Ok(Some(html)) = cache.get_html(&cache_key).await {
        return Html(html).into_response();
    }

    // 缓存未命中，生成 HTML
    let html_template = match render_blog_detail(vid.clone(), manifest.clone()).await {
        Ok(template) => template,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", e)
            ).into_response();
        }
    };

    // 将渲染后的 HTML 存入缓存
    let html_string = html_template.0.render().unwrap_or_default();
    
    #[cfg(debug_assertions)]
    let ttl_seconds = 1;
    #[cfg(not(debug_assertions))]
    let ttl_seconds = 3600;
    
    if let Err(e) = cache.set_html(
        cache_key,
        html_string.as_str(),
        ttl_seconds
    ).await {
        eprintln!("Failed to cache HTML: {}", e);
    }

    html_template.into_response()
}

pub fn blog_routes() -> Router<AppContext> {
    Router::new()
        .route("/blog/", get(blog_list))
        .route("/cat/:cat_vid/", get(blog_list_by_category))
        .route("/p/:vid/", get(blog_detail))
}

