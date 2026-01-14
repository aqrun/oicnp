use axum::{
    Router,
    routing::get,
    extract::{Extension, State},
    response::IntoResponse,
};
use oic_core::AppContext;
use crate::views::render_home_index;
use crate::{cached, consts::HANDLER_CACHE_TIME};
use oic_cache::Cache;
use std::sync::Arc;

// 类型别名，帮助类型推导
type CacheExtension = Arc<Cache>;

async fn index(
    State(_ctx): State<AppContext>,
    Extension(cache): Extension<CacheExtension>,
) -> impl IntoResponse {
    cached!(
        &*cache,
        "home:index",
        render_home_index(),
        HANDLER_CACHE_TIME
    )
}

pub fn home_routes() -> Router<AppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
}