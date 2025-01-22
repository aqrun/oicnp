use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*,
    typings::{JsonRes, ListData},
    utils::get_api_prefix,
    models::users::{
        UserFilters,
        CreateUserReqParams,
        UpdateUserReqParams,
        DeleteUserReqParams,
    },
};
use serde_json::json;

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<UserFilters>,
) -> JsonRes<UserModel> {
    let uid = params.uid.unwrap_or(0);
    let res = UserModel::find_by_uid(&ctx.db, uid).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<UserFilters>,
) -> JsonRes<ListData<UserModel>> {
    let res = UserModel::find_list(&ctx.db, params)
        .await;
    JsonRes::from(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateUserReqParams>,
) -> JsonRes<UserModel> {
    let res = UserModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateUserReqParams>,
) -> JsonRes<i64> {
    let res = UserModel::update(&ctx.db, params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteUserReqParams>,
) -> JsonRes<i64> {
    let res = UserModel::delete(&ctx.db, params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "user").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add", post(add))
        .add("/update", post(update))
        .add("/remove", post(remove))
}
