#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{debug_handler, http::Uri};
use loco_rs::prelude::*;
use crate::views;
use oic_core::typings::JsonRes;

/// 前端入口
#[debug_handler]
pub async fn index(
    ViewEngine(v): ViewEngine<TeraView>,
    // State(ctx): State<AppContext>,
) -> Result<Response> {
    views::home::index(&v)
}

/// 后台管理入口
#[debug_handler]
pub async fn admin(
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    views::admin::index(&v)
}

pub async fn fallback(uri: Uri) -> JsonRes<String> {
    JsonRes::err(format!("接口不存在 {}", uri.path()))
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(index))
        .add("/admin", get(admin))
}
