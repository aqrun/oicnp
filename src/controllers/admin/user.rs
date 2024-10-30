use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::entities::prelude::*;
use oic_core::utils::get_admin_prefix;
use serde_json::json;

const API_PREFIX: &'static str = "user";

#[debug_handler]
pub async fn get_one() -> Result<Response> {
    let res: serde_json::Value = json!({
      "name": "alex"
    });
    format::json(res)
}

#[debug_handler]
pub async fn list() -> Result<Response> {
    let res: serde_json::Value = json!({
      "name": "alex"
    });
    format::json(res)
}

#[debug_handler]
pub async fn add() -> Result<Response> {
    let res: serde_json::Value = json!({
      "name": "alex"
    });
    format::json(res)
}

#[debug_handler]
pub async fn update() -> Result<Response> {
    let res: serde_json::Value = json!({
      "name": "alex"
    });
    format::json(res)
}

#[debug_handler]
pub async fn remove() -> Result<Response> {
    let res: serde_json::Value = json!({
      "name": "alex"
    });
    format::json(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_admin_prefix(API_PREFIX).as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add", post(add))
        .add("/update", post(update))
        .add("/remove", post(remove))
}
