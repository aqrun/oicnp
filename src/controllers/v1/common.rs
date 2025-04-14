use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    utils::get_api_prefix,
    typings::JsonRes,
};

#[debug_handler]
pub async fn info() -> JsonRes {
    JsonRes::ok(String::from("Admin Api success"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "").as_str())
        .add("/info", get(info).post(info))
}