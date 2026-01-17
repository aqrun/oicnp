use axum::{
    Router,
    routing::get,
    extract::State,
    response::IntoResponse,
};
use crate::views::render_home_index;
use crate::{cached, consts::HANDLER_CACHE_TIME, WebAppContext};

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

pub fn home_routes() -> Router<WebAppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
}