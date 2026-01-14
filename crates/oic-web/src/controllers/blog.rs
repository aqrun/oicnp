use axum::{
    Router,
    routing::get,
    extract::{Extension, State, Path},
    response::IntoResponse,
};
use oic_core::AppContext;
use crate::views::{render_blog_list, render_blog_detail};
use crate::{cached, consts::HANDLER_CACHE_TIME};
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
    cached!(
        &*cache,
        "blog:list",
        render_blog_list(None, manifest.clone()),
        HANDLER_CACHE_TIME
    )
}

/// 分类博客列表页
async fn blog_list_by_category(
    Path(cat_vid): Path<String>,
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let cache_key = format!("blog:list:cat:{}", cat_vid);
    cached!(
        &*cache,
        &cache_key,
        render_blog_list(Some(cat_vid.clone()), manifest.clone()),
        HANDLER_CACHE_TIME
    )
}

/// 博客详情页
async fn blog_detail(
    Path(vid): Path<String>,
    State(_ctx): State<AppContext>,
    Extension(manifest): Extension<ManifestExtension>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    let cache_key = format!("blog:detail:{}", vid);
    cached!(
        &*cache,
        &cache_key,
        render_blog_detail(vid.clone(), manifest.clone()),
        HANDLER_CACHE_TIME
    )
}

pub fn blog_routes() -> Router<AppContext> {
    Router::new()
        .route("/blog/", get(blog_list))
        .route("/cat/{cat_vid}/", get(blog_list_by_category))
        .route("/p/{vid}/", get(blog_detail))
}

