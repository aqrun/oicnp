use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::nodes::{
        CreateNodeReqParams,
        UpdateNodeReqParams,
        DeleteNodeReqParams,
        NodeFilters,
    },
    utils::get_admin_prefix,
    typings::{JsonRes, ListData},
};

const API_PREFIX: &'static str = "node";

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<NodeFilters>,
) -> JsonRes<NodeModel> {
    let nid = params.nid.unwrap_or(0);
    let res = NodeModel::find_by_id(&ctx.db, nid).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<NodeFilters>,
) -> JsonRes<ListData<NodeModel>> {
    let res = NodeModel::find_list(&ctx.db, params)
        .await;
    JsonRes::from(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateNodeReqParams>,
) -> JsonRes<NodeModel> {
    let res = NodeModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreateNodeReqParams>>,
) -> JsonRes<String> {
    let res = NodeModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateNodeReqParams>,
) -> JsonRes<i64> {
    let res = NodeModel::update(&ctx.db, params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteNodeReqParams>,
) -> JsonRes<i64> {
    let res = NodeModel::delete(&ctx.db, params).await;

    JsonRes::from(res)
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
