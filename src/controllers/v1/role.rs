use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::roles::{
        CreateRoleReqParams,
        UpdateRoleReqParams,
        DeleteRoleReqParams,
        RoleFilters,
    },
    utils::get_api_prefix,
    typings::{JsonRes, ListData},
    ModelCrudHandler,
};

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<RoleFilters>,
) -> JsonRes<RoleModel> {
    let id = params.role_id.unwrap_or(0);
    let res = RoleModel::find_by_id(&ctx.db, id).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<RoleFilters>,
) -> JsonRes<ListData<RoleModel>> {
    let res = RoleModel::find_list(&ctx.db, &params)
        .await;
    JsonRes::from(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateRoleReqParams>,
) -> JsonRes<i64> {
    let res = RoleModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreateRoleReqParams>>,
) -> JsonRes<String> {
    let res = RoleModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateRoleReqParams>,
) -> JsonRes<i64> {
    let res = RoleModel::update(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteRoleReqParams>,
) -> JsonRes<i64> {
    let res = RoleModel::delete_one(&ctx.db, &params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "role").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add", post(add))
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
}
