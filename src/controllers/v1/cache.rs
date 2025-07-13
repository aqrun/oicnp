use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::caches::{
        DeleteCacheReqParams,
        CacheFilters,
        CacheScopeModel,
    },
    utils::get_api_prefix,
    typings::{JsonRes, Pagination},
    ModelCrudHandler,
};

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<CacheFilters>,
) -> JsonRes<CacheModel> {
    let mut res: ModelResult<CacheModel> = Err(ModelError::Any(String::from("参数不能为空").into()));
    
    if let Some(id) = params.id {
        res = CacheModel::find_by_id(&ctx.db, id).await;
    }

    if let Some(cache_key) = &params.cache_key {
        res = CacheModel::find_by_vid(&ctx.db, cache_key.as_str()).await;
    }

    match res {
        Ok(data) => {
            // 使用两个数据的元组指定最终 JSON 数据 key
            JsonRes::from((data, "cache"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<CacheFilters>,
) -> JsonRes<Vec<CacheModel>> {
    let (caches, total) = match CacheModel::find_list(&ctx.db, &params).await {
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
    JsonRes::from((caches, pager, "caches"))
}

#[debug_handler]
pub async fn scope_list(
    State(ctx): State<AppContext>,
) -> JsonRes<Vec<CacheScopeModel>> {
    let scopes = match CacheModel::find_scope_list(&ctx.db).await {
        Ok(res) => res,
        Err(err) => return JsonRes::err(err),
    };

    JsonRes::from((scopes, "scopes"))
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteCacheReqParams>,
) -> JsonRes<i64> {
    let res = CacheModel::delete_one(&ctx.db, &params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "cache").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/scope-list", post(scope_list))
        .add("/remove", post(remove))
}
