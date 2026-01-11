use axum::{
    Router,
    routing::get,
    extract::{Extension, State},
    response::IntoResponse,
};
use oic_core::AppContext;
use crate::views::render_home_index;
use crate::cached;
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
    cached!(&*cache, "home:index", render_home_index(manifest.clone()))
}

pub fn home_routes() -> Router<AppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
}