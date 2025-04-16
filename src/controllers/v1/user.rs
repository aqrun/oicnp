use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*,
    typings::{JsonRes, Pagination},
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

    let res = if uid > 0 {
        UserModel::find_by_uid(&ctx.db, uid).await
    } else {
        UserModel::find_by_uuid(&ctx.db, uuid.as_str()).await
    };

    match res {
        Ok(data) => {
            // 使用两个数据的元组指定最终 JSON 数据 key
            JsonRes::from((data, "user"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<UserFilters>,
) -> JsonRes<Vec<UserModel>> {
    let (users, total) = match UserModel::find_list(&ctx.db, &params).await {
        Ok(res) => res,
        Err(err) => return JsonRes::err(err),
    };
    
    // 分页数据
    let pager = Pagination {
        total,
        page: params.page.unwrap_or(1),
        page_size: params.page_size.unwrap_or(10),
    };

    // 使用传递三个数据的元组指定最终 JSON 数据 key
    JsonRes::from((users, pager, "users"))
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
