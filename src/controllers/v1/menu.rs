use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*,
    models::menus::{
        MenuFilters,
        CreateMenuReqParams,
        UpdateMenuReqParams,
        DeleteMenuReqParams,
    },
};
use oic_core::typings::{JsonRes, ListData};
use oic_core::utils::get_api_prefix;
use serde_json::Value;

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<MenuFilters>,
) -> JsonRes<MenuModel> {
    let id = params.id.unwrap_or(0);
    let res = MenuModel::find_by_id(&ctx.db, id).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<MenuFilters>,
) -> JsonRes<ListData<MenuModel>> {
    let res = MenuModel::find_list(&ctx.db, params)
        .await;
    JsonRes::from(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateMenuReqParams>,
) -> JsonRes<MenuModel> {
    let res = MenuModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreateMenuReqParams>>,
) -> JsonRes<String> {
    let res = MenuModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateMenuReqParams>,
) -> JsonRes<i32> {
    let res = MenuModel::update(&ctx.db, params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteMenuReqParams>,
) -> JsonRes<i64> {
    let res = MenuModel::delete(&ctx.db, params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "menu").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
}