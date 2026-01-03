use axum::{
    Router,
    response::IntoResponse,
    routing::get,
    extract::{Extension, State},
};
use oic_core::AppContext;
use crate::models::HtmlTemplate;
use crate::views::HomeTemplate;
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::{ManifestChunk, ViteAssets};

async fn index(
    State(_ctx): State<AppContext>,
    manifest: Extension<Arc<HashMap<String, ManifestChunk>>>
) -> impl IntoResponse {
    let m: HashMap<String, ManifestChunk> = (*manifest.0).clone();
    let assets: ViteAssets = ManifestChunk::get_assets_by_name(m, "main");
    let template = HomeTemplate {
        name: "World".to_string(),
        assets,
    };
    HtmlTemplate(template)
}

pub fn home_routes() -> Router<AppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
}