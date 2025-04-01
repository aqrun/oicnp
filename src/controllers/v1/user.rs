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
    ModelCrudHandler,
};

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<UserFilters>,
) -> JsonRes<UserModel> {
    let uid = params.uid.unwrap_or(0);
    let uuid = params.uuid.unwrap_or(String::from(""));

    if uid > 0 {
        let res = UserModel::find_by_uid(&ctx.db, uid).await;
    
        return JsonRes::from(res);
    }

    let res = UserModel::find_by_uuid(&ctx.db, uuid.as_str()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<UserFilters>,
) -> JsonRes<ListData<UserModel>> {
    let res = UserModel::find_list(&ctx.db, &params)
        .await;
    JsonRes::from(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateUserReqParams>,
) -> JsonRes<i64> {
    let res = UserModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreateUserReqParams>>,
) -> JsonRes<String> {
    let res = UserModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateUserReqParams>,
) -> JsonRes<i64> {
    let res = UserModel::update(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteUserReqParams>,
) -> JsonRes<i64> {
    let res = UserModel::delete_one(&ctx.db, &params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "user").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add", post(add))
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
}
