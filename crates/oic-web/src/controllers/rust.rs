use crate::{cached, WebAppContext};
use crate::views::render_rust_home;
use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
};

/// rust首页
async fn rust_home(
    State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
    let cache_key = "rust:home";
    cached!(
        &ctx.cache,
        &cache_key,
        render_rust_home(&ctx),
        ctx.config.handler_cache_time
    )
}

pub fn rust_routes() -> Router<WebAppContext> {
    Router::new().route("/rust", get(rust_home))
}
