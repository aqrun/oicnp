use axum::{
  Router,
  routing::get,
  extract::{State, Path, Query},
  response::IntoResponse,
};
use oic_core::middleware::HtmxRequest;
use crate::views::poetry::{render_poetry_home, render_poetry_list, render_poetry_detail};
use crate::{cached, consts::HANDLER_CACHE_TIME, WebAppContext};
use crate::models::poetry::PoetryListParams;

/// 诗词首页
async fn poetry_home(
  State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
  let cache_key = "poetry:home";
  cached!(
      &ctx.cache,
      &cache_key,
      render_poetry_home(&ctx),
      HANDLER_CACHE_TIME
  )
}

/// 分类诗词列表页
async fn poetry_list_by_category(
  Path(cat_vid): Path<String>,
  State(ctx): State<WebAppContext>,
  Query(params): Query<PoetryListParams>,
  htmx: HtmxRequest,
) -> impl IntoResponse {
  let mut page = 1;
  let list_key = if htmx.is_htmx { "htmx_list" } else { "list" };

  if let Some(x) = params.page {
      page = x;
  }

  let cache_key = format!("poetry:cat:{}:{}:{}", list_key, cat_vid, page);
  let new_params = PoetryListParams {
      cat_vid: Some(cat_vid),
      page: Some(page),
      ..Default::default()
  };
  cached!(
      &ctx.cache,
      &cache_key,
      render_poetry_list(&ctx, &new_params, &htmx),
      HANDLER_CACHE_TIME
  )
}

/// 诗词详情页
async fn poetry_detail(
  Path(uuid): Path<String>,
  State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
  let cache_key = format!("poetry:detail:{}", uuid);
  cached!(
      &ctx.cache,
      &cache_key,
      render_poetry_detail(&ctx, uuid.clone()),
      HANDLER_CACHE_TIME
  )
}

pub fn poetry_routes() -> Router<WebAppContext> {
  Router::new()
      .route("/poetry", get(poetry_home))
      .route("/poetry/cat/{cat_vid}", get(poetry_list_by_category))
      .route("/poetry/n/{uuid}", get(poetry_detail))
}

