use std::sync::Arc;
use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::user_online::{UserOnlineFilters, DeleteUserOnlineReqParams},
    utils::get_api_prefix,
    typings::{JsonRes, Pagination},
    ModelCrudHandler,
    services::cache::OicCache,
};

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<UserOnlineFilters>,
) -> JsonRes<UserOnlineModel> {
    let id = params.uid.unwrap_or(0);
    let res = UserOnlineModel::find_by_id(&ctx.db, id).await;

    match res {
        Ok(data) => {
            // 使用两个数据的元组指定最终 JSON 数据 key
            JsonRes::from((data, "online"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<UserOnlineFilters>,
) -> JsonRes<Vec<UserOnlineModel>> {
    // 数据清理
    let _ = UserOnlineModel::refresh(&ctx.db).await;

    let (online_list, total) = match UserOnlineModel::find_list(&ctx.db, &params).await {
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
    JsonRes::from((online_list, pager, "onlineList"))
}

pub async fn force_logout(
    State(ctx): State<AppContext>,
    Json(params): Json<UserOnlineFilters>,
) -> JsonRes<String> {
    let cache = match ctx.shared_store.get::<Arc<OicCache>>() {
        Some(cache) => cache,
        None => {
            return JsonRes::err(String::from("Cache not found"));
        },
    };
    
    let token_id = params.token_id.unwrap_or(String::from(""));
    let cache_key = format!("session-{}", token_id.as_str());
    let _ = cache.remove(cache_key.as_str()).await;

    let del_params = DeleteUserOnlineReqParams {
        uid: params.uid,
        ..Default::default()
    };
    let _ = UserOnlineModel::delete_one(&ctx.db, &del_params).await;

    JsonRes::from((String::from("success"), "res"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "online").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/force_logout", post(force_logout))
}
