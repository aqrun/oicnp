use axum::{
    Router,
    routing::get,
    extract::{State, Query},
    response::IntoResponse,
    http::StatusCode,
};
use crate::views::{render_home_index, render_out_link};
use crate::{cached, consts::HANDLER_CACHE_TIME, WebAppContext};
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
        HANDLER_CACHE_TIME
    )
}

async fn out_link(
    Query(params): Query<OutLinkParams>,
) -> impl IntoResponse {
    // 解码 URL 参数
    let target_url = match urlencoding::decode(&params.target) {
        Ok(decoded) => decoded.to_string(),
        Err(_) => params.target.clone(), // 如果解码失败，使用原始值
    };
    
    match render_out_link(target_url) {
        Ok(bytes) => bytes.into_response(),
        Err(e) => {
            tracing::error!("Failed to render out link page: {}", e);
            // 返回一个简单的错误页面
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

pub fn home_routes() -> Router<WebAppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
        .route("/link", get(out_link))
}