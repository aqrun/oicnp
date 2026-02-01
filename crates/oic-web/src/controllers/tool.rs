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

/// 工具列表页
async fn tool_list(
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

  let cache_key = format!("tool:{}:{}:{}", list_key, cat_vid, page);

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

pub fn tool_routes() -> Router<WebAppContext> {
  Router::new()
      .route("/tool", get(tool_list))
}

