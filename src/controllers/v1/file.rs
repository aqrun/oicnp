use std::sync::Arc;
use axum::{
    debug_handler,
    extract::Multipart,
};
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*, 
    models::files::{
        CreateFileReqParams,
        UpdateFileReqParams,
        DeleteFileReqParams,
        FileFilters,
        UploadFileRes,
    },
    utils::get_api_prefix,
    typings::{JsonRes, Pagination},
    ModelCrudHandler,
    prelude::Settings,
    services::file::{
        store_file_local,
        store_file_oss,
    },
};
use futures::TryStreamExt;
use std::io;
use tokio_util::io::StreamReader;

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<FileFilters>,
) -> JsonRes<FileModel> {
    let id = params.file_id.unwrap_or(0);
    let res = FileModel::find_by_id(&ctx.db, id).await;

    match res {
        Ok(data) => {
            // 使用两个数据的元组指定最终 JSON 数据 key
            JsonRes::from((data, "file"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<FileFilters>,
) -> JsonRes<Vec<FileModel>> {
    let (files, total) = match FileModel::find_list(&ctx.db, &params).await {
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
    JsonRes::from((files, pager, "files"))
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateFileReqParams>,
) -> JsonRes<i64> {
    let res = FileModel::create(&ctx.db, &params).await;

    JsonRes::from(res)
}

/// 批量添加
#[debug_handler]
pub async fn add_multi(
    State(ctx): State<AppContext>,
    Json(params): Json<Vec<CreateFileReqParams>>,
) -> JsonRes<String> {
    let res = FileModel::create_multi(&ctx.db, params.as_slice()).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateFileReqParams>,
) -> JsonRes<i64> {
    let res = FileModel::update(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn remove(
    State(ctx): State<AppContext>,
    Json(params): Json<DeleteFileReqParams>,
) -> JsonRes<i64> {
    let res = FileModel::delete_one(&ctx.db, &params).await;

    JsonRes::from(res)
}

#[debug_handler]
pub async fn upload(
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> JsonRes<UploadFileRes> {
    let settings = match ctx.shared_store.get::<Arc<Settings>>() {
        Some(s) => s,
        None => {
            return JsonRes::err(String::from("Storage 配置参数不存在"));
        },
    };
    let storage_cfg = settings.storage.clone();

    let mut file_name = String::from("");
    let mut file_size = 0;
    let mut file_type = String::from("");
    let mut storage = String::from("local");
    let mut uri = String::from("");
    // 图床地址
    let mut link = String::from("");
    let mut file_req_params = CreateFileReqParams::default();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        
        if name.as_str().eq("name") {
            file_name = field.text().await.unwrap_or("".to_string());
        } else if name.as_str().eq("size") {
            let size_str = field.text().await.unwrap_or("".to_string());
            file_size = size_str.parse::<i32>().unwrap_or(0);
        } else if name.as_str().eq("type") {
            file_type = field.text().await.unwrap_or("".to_string());
        } else if name.as_str().eq("storage") {
            storage = field.text().await.unwrap_or("local".to_string());

            if !storage_cfg.driver.as_str().eq(storage.as_str()) {
                return JsonRes::err(format!("指定 Storage 配置参数不匹配： {}", storage));
            }
        } else if name.as_str().eq("link") {
            link = field.text().await.unwrap_or("".to_string());
        } else if name.as_str().eq("file") {
            file_name = if let Some(filename) = field.file_name() {
                filename.to_string()
            } else {
                continue;
            };

            let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
            let body_reader = StreamReader::new(body_with_io_error);

            futures::pin_mut!(body_reader);

            file_req_params.filename = Some(String::from(file_name.as_str()));
            file_req_params.storage = Some(String::from(storage.as_str()));
            file_req_params.mime = Some(String::from(file_type.as_str()));
            file_req_params.link = Some(String::from(link.as_str()));
            file_req_params.size = Some(file_size);
            
            // 本地存储
            if storage.as_str().eq("local") {
                uri = match store_file_local(
                    body_reader, 
                    &storage_cfg, 
                    &file_req_params
                ).await {
                    Ok(res) => res,
                    Err(err) => {
                        return JsonRes::err(err.to_string());
                    }
                };
            } else if storage_cfg.driver.as_str().eq("oss") {
                uri = match store_file_oss(body_reader, &storage_cfg, &file_req_params).await {
                    Ok(res) => res,
                    Err(err) => {
                        return JsonRes::err(err.to_string());
                    }
                };
            }
        }
    }

    file_req_params.uri = Some(uri);
    let res = match FileModel::create(&ctx.db, &file_req_params).await {
        Ok(res) => res,
        Err(err) => {
            return JsonRes::err(err.to_string());
        }
    };
    let res_file = match FileModel::find_by_id(&ctx.db, res).await {
        Ok(res) => res,
        Err(err) => {
            return JsonRes::err(err.to_string());
        }
    };

    // 转换为接口返回数据
    let mut res = UploadFileRes::from(res_file);
    res.url = format!("{}/{}", storage_cfg.uri, res.url);
    
    JsonRes::from((res, "file"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "file").as_str())
        .add("/one", post(get_one))
        .add("/list", post(list))
        .add("/add", post(add))
        .add("/add-multi", post(add_multi))
        .add("/update", post(update))
        .add("/remove", post(remove))
        .add("/upload", post(upload))
}
