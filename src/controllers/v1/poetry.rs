use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    models::poetry::{
        PoetryFilters,
        CreatePoetryReqParams,
        UpdatePoetryReqParams,
        DeletePoetryReqParams,
        PoetryModel,
        PoetryListPageDataResponse,
    },
    utils::get_api_prefix,
    typings::{JsonRes, Pagination},
    ModelCrudHandler,
    prelude::get_poetry_db,
};

#[debug_handler]
pub async fn get_one(
    State(_ctx): State<AppContext>,
    Json(params): Json<PoetryFilters>,
) -> JsonRes<PoetryModel> {
    let db = get_poetry_db().await;
    let id = params.id.unwrap_or(0);
    let res = PoetryModel::find_by_id(db, id as i64).await;

    match res {
        Ok(data) => {
            // 使用两个数据的元组指定最终 JSON 数据 key
            JsonRes::from((data, "poetry"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

#[debug_handler]
pub async fn list(
    State(_ctx): State<AppContext>,
    Json(params): Json<PoetryFilters>,
) -> JsonRes<Vec<PoetryModel>> {
    let db = get_poetry_db().await;
    let (poetry, total) = match PoetryModel::find_list(db, &params).await {
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
    JsonRes::from((poetry, pager, "poetry"))
}

#[debug_handler]
pub async fn add(
    State(_ctx): State<AppContext>,
    Json(params): Json<CreatePoetryReqParams>,
) -> JsonRes<i64> {
    let db = get_poetry_db().await;
    let res = PoetryModel::create(db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(_ctx): State<AppContext>,
    Json(params): Json<Vec<CreatePoetryReqParams>>,
) -> JsonRes<String> {
    let db = get_poetry_db().await;
    let res = PoetryModel::create_multi(db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(_ctx): State<AppContext>,
    Json(params): Json<UpdatePoetryReqParams>,
) -> JsonRes<i64> {
    let db = get_poetry_db().await;
    let res = PoetryModel::update(db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(_ctx): State<AppContext>,
    Json(params): Json<DeletePoetryReqParams>,
) -> JsonRes<i64> {
    let db = get_poetry_db().await;
    let res = PoetryModel::delete_one(db, &params).await;

    JsonRes::from(res)
}

pub async fn get_list_page_data(
    Json(params): Json<PoetryFilters>,
) -> JsonRes<PoetryListPageDataResponse> {
    let db = get_poetry_db().await;
    let res = PoetryModel::get_list_page_data(db, params).await;

    match res {
        Ok(data) => {
            // 返回 poetry 列表作为 entries，chapters 数据包含在响应中但前端目前只使用 entries
            JsonRes::from((data, "entry"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

pub async fn get_list_with_chapters(
    Json(params): Json<PoetryFilters>,
) -> JsonRes<PoetryListPageDataResponse> {
    let db = get_poetry_db().await;
    let res = PoetryModel::find_list_with_chapters(db, &params).await;

    match res {
        Ok(data) => {
            JsonRes::from((data, "entry"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "poetry").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add", post(add))
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
        .add("/list-page-data", post(get_list_page_data))
        .add("/list-with-chapters", post(get_list_with_chapters))
}
