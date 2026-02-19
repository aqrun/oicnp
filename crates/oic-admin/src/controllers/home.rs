use axum::{
    Router,
    routing::get,
    extract::{State, Query},
    response::{IntoResponse, Html},
    http::StatusCode,
};
use crate::views::{
    render_home_index,
};
use crate::{cached, WebAppContext};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OutLinkParams {
    pub target: String,
}

async fn index(
    State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
    cached!(
        &ctx.cache,
        "home:index",
        render_home_index(&ctx),
        ctx.config.handler_cache_time
    )
}

pub fn home_routes() -> Router<WebAppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
}