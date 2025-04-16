use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*,
    models::menus::{
        MenuFilters,
        CreateMenuReqParams,
        UpdateMenuReqParams,
        DeleteMenuReqParams,
        MenuTreeItem,
    },
    ModelCrudHandler,
};
use oic_core::typings::{JsonRes, Pagination};
use oic_core::utils::get_api_prefix;

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<MenuFilters>,
) -> JsonRes<MenuModel> {
    let id = params.id.unwrap_or(0);
    let res = MenuModel::find_by_id(&ctx.db, id as i64).await;

    match res {
        Ok(data) => {
            // 使用两个数据的元组指定最终 JSON 数据 key
            JsonRes::from((data, "menu"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<MenuFilters>,
) -> JsonRes<Vec<MenuModel>> {
    let (menus, total) = match MenuModel::find_list(&ctx.db, &params).await {
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
    JsonRes::from((menus, pager, "menus"))
}

/// 生成树形数据
#[debug_handler]
pub async fn get_tree(
    State(ctx): State<AppContext>,
    Json(params): Json<MenuFilters>,
) -> JsonRes<MenuTreeItem> {
    let res = MenuModel::find_tree(&ctx.db, params).await;
    JsonRes::from(res)
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateMenuReqParams>,
) -> JsonRes<i64> {
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
) -> JsonRes<i64> {
    let res = MenuModel::update(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteMenuReqParams>,
) -> JsonRes<i64> {
    let res = MenuModel::delete_one(&ctx.db, &params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "menu").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/tree", post(get_tree))
        .add("/add", post(add))
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
}