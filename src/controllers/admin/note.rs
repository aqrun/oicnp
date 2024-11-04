use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::notes::CreateNoteReqParams, 
    utils::{get_admin_prefix, catch_err},
    typings::JsonRes,
};
use serde_json::json;

const API_PREFIX: &'static str = "note";

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateNoteReqParams>,
) -> JsonRes<NoteModel> {
    let res = NoteModel::insert(&ctx.db, &params).await;

    match res {
        Ok(res) => JsonRes::ok(res),
        Err(err) => JsonRes::err(err)
    }
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
    Json(params): Json<CreateNoteReqParams>,
) -> JsonRes<NoteModel> {
    if let Err(err) = catch_err(params.validate()) {
        return JsonRes::err(err);
    }

    let res = NoteModel::insert(&ctx.db, &params).await;

    match res {
        Ok(res) => JsonRes::ok(res),
        Err(err) => JsonRes::err(err),
    }
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
