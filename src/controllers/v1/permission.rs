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
) -> JsonRes<PermissionModel> {
    let id = params.permission_id.unwrap_or(0);
    let res = PermissionModel::find_by_id(&ctx.db, id).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<PermissionFilters>,
) -> JsonRes<ListData<PermissionModel>> {
    let (permissions, total) = match PermissionModel::find_list(&ctx.db, &params).await {
        Ok(res) => res,
        Err(err) => return JsonRes::err(err),
    };
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    
    let list_data = ListData {
        data: permissions,
        total,
        page,
        page_size,
    };
    JsonRes::from((list_data, "permissions"))
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreatePermissionReqParams>,
) -> JsonRes<i64> {
    let res = PermissionModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreatePermissionReqParams>>,
) -> JsonRes<String> {
    let res = PermissionModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdatePermissionReqParams>,
) -> JsonRes<i64> {
    let res = PermissionModel::update(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeletePermissionReqParams>,
) -> JsonRes<i64> {
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
