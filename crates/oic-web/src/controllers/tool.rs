use axum::{
  Router,
  routing::get,
  extract::{State, Path, Query},
  response::IntoResponse,
};
use crate::views::{render_tool_list};
use crate::{cached, WebAppContext};
use crate::models::tool::ToolListParams;

/// 工具列表页
async fn tool_list(
  State(ctx): State<WebAppContext>,
  Query(params): Query<ToolListParams>,
) -> impl IntoResponse {
  let cache_key = "web:tool:list:all";

  cached!(
      &ctx.cache,
      &cache_key,
      render_tool_list(&ctx, &params),
      ctx.config.handler_cache_time
  )
}

/// 分类工具列表页
async fn tool_list_by_category(
  Path(cat_vid): Path<String>,
  State(ctx): State<WebAppContext>,
) -> impl IntoResponse {
  let cache_key = format!("web:tool:list:cat:{}", cat_vid);
  let new_params = ToolListParams {
      cat_vid: Some(cat_vid),
      is_category_page: Some(true),
      ..Default::default()
  };
  cached!(
      &ctx.cache,
      &cache_key,
      render_tool_list(&ctx, &new_params),
      ctx.config.handler_cache_time
  )
}

pub fn tool_routes() -> Router<WebAppContext> {
  Router::new()
      .route("/tool", get(tool_list))
      .route("/tool/cat/{cat_vid}", get(tool_list_by_category))
}

