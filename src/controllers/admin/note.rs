use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::notes::{
        CreateNoteReqParams,
        NoteFilters,
    },
    utils::get_admin_prefix,
    typings::{JsonRes, ListData},
};
use serde_json::json;

const API_PREFIX: &'static str = "note";

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<NoteFilters>,
) -> JsonRes<NoteModel> {
    let id = params.id.unwrap_or(0);
    let res = NoteModel::find_by_id(&ctx.db, id).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<NoteFilters>,
) -> JsonRes<ListData<NoteModel>> {
    let res = NoteModel::find_list(&ctx.db, params)
        .await;
    JsonRes::from(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateNoteReqParams>,
) -> JsonRes<NoteModel> {
    let res = NoteModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreateNoteReqParams>>,
) -> JsonRes<String> {
    let res = NoteModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
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
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
}
