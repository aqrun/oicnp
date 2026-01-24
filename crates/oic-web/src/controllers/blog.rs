use axum::{
    Router,
    routing::get,
    extract::{State, Path, Query},
    response::IntoResponse,
};
use oic_core::middleware::HtmxRequest;
use crate::views::{render_blog_list, render_blog_detail};
use crate::{cached, consts::HANDLER_CACHE_TIME, WebAppContext};
use crate::models::blog::BlogListParams;

/// 博客列表页
async fn blog_list(
    State(ctx): State<WebAppContext>,
    Query(params): Query<BlogListParams>,
    htmx: HtmxRequest,
) -> impl IntoResponse {
    let mut cat_vid = String::from("all");
    let mut page = 1;
    let list_key = if htmx.is_htmx { "htmx_list" } else { "list" };

    if let Some(x) = &params.cat_vid {
        cat_vid = String::from(x);
    }
    if let Some(x) = params.page {
        page = x;
    }

    let cache_key = format!("blog:{}:{}:{}", list_key, cat_vid, page);

    cached!(
        &ctx.cache,
        &cache_key,
        render_blog_list(&ctx, &params, &htmx),
        HANDLER_CACHE_TIME
    )
}

/// 分类博客列表页
async fn blog_list_by_category(
    Path(cat_vid): Path<String>,
    State(ctx): State<WebAppContext>,
    Query(params): Query<BlogListParams>,
    htmx: HtmxRequest,
) -> impl IntoResponse {
    let mut page = 1;
    let list_key = if htmx.is_htmx { "htmx_list" } else { "list" };

    if let Some(x) = params.page {
        page = x;
    }

    let cache_key = format!("blog:cat:{}:{}:{}", list_key, cat_vid, page);
    let new_params = BlogListParams {
        cat_vid: Some(cat_vid),
        page: Some(page),
        is_category_page: Some(true),
        ..Default::default()
    };
    cached!(
        &ctx.cache,
        &cache_key,
        render_blog_list(&ctx, &new_params, &htmx),
        HANDLER_CACHE_TIME
    )
}

/// 标签博客列表页
async fn blog_list_by_tag(
    Path(tag_vid): Path<String>,
    State(ctx): State<WebAppContext>,
    Query(params): Query<BlogListParams>,
    htmx: HtmxRequest,
) -> impl IntoResponse {
    let mut page = 1;
    let list_key = if htmx.is_htmx { "htmx_list" } else { "list" };

    if let Some(x) = params.page {
        page = x;
    }

    let cache_key = format!("blog:tag:{}:{}:{}", list_key, tag_vid, page);
    let new_params = BlogListParams {
        tag_vid: Some(tag_vid),
        page: Some(page),
        is_tag_page: Some(true),
        ..Default::default()
    };
    cached!(
        &ctx.cache,
        &cache_key,
        render_blog_list(&ctx, &new_params, &htmx),
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
        .route("/tag/{tag_vid}/", get(blog_list_by_tag))
        .route("/p/{vid}/", get(blog_detail))
}

