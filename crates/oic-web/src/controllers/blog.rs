use axum::{
    Router,
    routing::get,
    extract::{State, Path},
    response::IntoResponse,
};
use crate::views::{render_blog_list, render_blog_detail};
use crate::{cached, consts::HANDLER_CACHE_TIME, WebAppContext};

/// 博客列表页
async fn blog_list(
    State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
    cached!(
        &ctx.cache,
        "blog:list",
        render_blog_list(&ctx, None),
        HANDLER_CACHE_TIME
    )
}

/// 分类博客列表页
async fn blog_list_by_category(
    Path(cat_vid): Path<String>,
    State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
    let cache_key = format!("blog:list:cat:{}", cat_vid);
    cached!(
        &ctx.cache,
        &cache_key,
        render_blog_list(&ctx, Some(cat_vid.clone())),
        HANDLER_CACHE_TIME
    )
}

/// 博客详情页
async fn blog_detail(
    Path(vid): Path<String>,
    State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
    let cache_key = format!("blog:detail:{}", vid);
    cached!(
        &ctx.cache,
        &cache_key,
        render_blog_detail(&ctx, vid.clone()),
        HANDLER_CACHE_TIME
    )
}

pub fn blog_routes() -> Router<WebAppContext> {
    Router::new()
        .route("/blog/", get(blog_list))
        .route("/cat/{cat_vid}/", get(blog_list_by_category))
        .route("/p/{vid}/", get(blog_detail))
}

