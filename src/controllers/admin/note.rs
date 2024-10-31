use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::notes::CreateNoteParams, 
    utils::{get_admin_prefix, catch_err},
    typings::JsonResponse,
};
use serde_json::json;

const API_PREFIX: &'static str = "note";

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateNoteParams>,
) -> Result<Response> {
    let res = NoteModel::insert(&ctx.db, &params).await;

    let res_data = match res {
        Ok(res) => JsonResponse::success(json!(res)),
        Err(err) => {
            JsonResponse::error(json!("400"), json!(format!("{}", err)))
        }
    };
    format::json(res_data)
}

#[debug_handler]
pub async fn list() -> Result<Response> {
    let res: serde_json::Value = json!({
      "name": "alex"
    });
    format::json(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateNoteParams>,
) -> Result<Response> {
    if let Err(err) = catch_err(params.validate()) {
        let msg = err.to_string();
        return format::json(JsonResponse::error(json!("400"), json!(msg)));
    }

    let res = NoteModel::insert(&ctx.db, &params).await;

    let res_data = match res {
        Ok(res) => JsonResponse::success(json!(res)),
        Err(err) => {
            JsonResponse::error(json!("400"), json!(format!("{}", err)))
        }
    };
    format::json(res_data)
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
