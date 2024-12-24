use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    utils::get_admin_prefix,
    typings::JsonRes,
};

#[debug_handler]
pub async fn info() -> JsonRes<String> {
    JsonRes::ok(String::from("Admin Api success"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_admin_prefix("").as_str())
        .add("/info", get(info))
}