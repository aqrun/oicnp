use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::permissions::{
        CreatePermissionReqParams,
        UpdatePermissionReqParams,
        DeletePermissionReqParams,
        PermissionFilters,
    },
    utils::get_api_prefix,
    typings::{JsonRes, ListData},
    ModelCrudHandler,
};

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<PermissionFilters>,
) -> JsonRes {
    let id = params.permission_id.unwrap_or(0);
    let res = PermissionModel::find_by_id(&ctx.db, id).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<PermissionFilters>,
) -> JsonRes {
    let res = PermissionModel::find_list(&ctx.db, &params)
        .await;
    JsonRes::from(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreatePermissionReqParams>,
) -> JsonRes {
    let res = PermissionModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreatePermissionReqParams>>,
) -> JsonRes {
    let res = PermissionModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdatePermissionReqParams>,
) -> JsonRes {
    let res = PermissionModel::update(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeletePermissionReqParams>,
) -> JsonRes {
    let res = PermissionModel::delete_one(&ctx.db, &params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "permission").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add", post(add))
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
}
