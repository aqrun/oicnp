use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::tags::{
        CreateTagReqParams,
        UpdateTagReqParams,
        DeleteTagReqParams,
        TagFilters,
    },
    utils::get_api_prefix,
    typings::{JsonRes, Pagination},
    ModelCrudHandler,
};

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<TagFilters>,
) -> JsonRes<TagModel> {
    let id = params.tag_id.unwrap_or(0);
    let res = TagModel::find_by_id(&ctx.db, id).await;

    match res {
        Ok(data) => {
            // 使用两个数据的元组指定最终 JSON 数据 key
            JsonRes::from((data, "tag"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<TagFilters>,
) -> JsonRes<Vec<TagModel>> {
    let (tags, total) = match TagModel::find_list(&ctx.db, &params).await {
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
    JsonRes::from((tags, pager, "tags"))
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateTagReqParams>,
) -> JsonRes<i64> {
    let res = TagModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreateTagReqParams>>,
) -> JsonRes<String> {
    let res = TagModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateTagReqParams>,
) -> JsonRes<i64> {
    let res = TagModel::update(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteTagReqParams>,
) -> JsonRes<i64> {
    let res = TagModel::delete_one(&ctx.db, &params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "tag").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add", post(add))
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
}
