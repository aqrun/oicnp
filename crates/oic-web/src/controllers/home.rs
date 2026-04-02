use axum::{
    Router,
    routing::get,
    extract::{State, Query},
    response::{IntoResponse, Html},
    http::StatusCode,
};
use crate::views::{
    render_home_index,
    render_out_link,
    render_about,
    render_contact,
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
        "web:home:index",
        render_home_index(&ctx),
        ctx.config.handler_cache_time
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
        Ok(bytes) => Html(bytes).into_response(),
        Err(e) => {
            tracing::error!("Failed to render out link page: {}", e);
            // 返回一个简单的错误页面
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

async fn about(
    State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
    cached!(
        &ctx.cache,
        "web:home:about",
        render_about(&ctx),
        ctx.config.handler_cache_time
    )
}

async fn contact(
    State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
    cached!(
        &ctx.cache,
        "web:home:contact",
        render_contact(&ctx),
        ctx.config.handler_cache_time
    )
}

pub fn home_routes() -> Router<WebAppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
        .route("/link", get(out_link))
        .route("/about", get(about))
        .route("/contact", get(contact))
}