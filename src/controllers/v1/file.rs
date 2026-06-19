use axum::{debug_handler, extract::Multipart};
use futures::TryStreamExt;
use loco_rs::prelude::*;
use oic_core::{
    entities::prelude::*,
    models::files::{
        CreateFileReqParams, DeleteFileReqParams, FileFilters, UpdateFileReqParams, UploadFileRes,
    },
    prelude::Settings,
    services::storage::{StorageProvider, StorageProviderFactory},
    typings::{JsonRes, Pagination},
    utils::get_api_prefix,
    ModelCrudHandler,
};
use std::io;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;

fn get_settings(ctx: &AppContext) -> Arc<Settings> {
    ctx.shared_store
        .get::<Arc<Settings>>()
        .unwrap_or_else(|| Arc::new(Settings::default()))
}

fn get_provider(ctx: &AppContext) -> Result<Arc<dyn StorageProvider>, String> {
    ctx.shared_store
        .get::<Arc<dyn StorageProvider>>()
        .ok_or_else(|| String::from("Storage Provider 未初始化"))
}

#[debug_handler]
pub async fn get_one(
    State(ctx): State<AppContext>,
    Json(params): Json<FileFilters>,
) -> JsonRes<UploadFileRes> {
    let provider = match get_provider(&ctx) {
        Ok(p) => p,
        Err(err) => return JsonRes::err(err),
    };

    let id = params.file_id.unwrap_or(0);
    let res = FileModel::find_by_id(&ctx.db, id).await;

    match res {
        Ok(data) => {
            let mut file = UploadFileRes::from(data);
            file.url = provider.public_url(&file.uri);
            JsonRes::from((file, "file"))
        }
        Err(err) => JsonRes::err(err),
    }
}

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Json(params): Json<FileFilters>,
) -> JsonRes<Vec<UploadFileRes>> {
    let provider = match get_provider(&ctx) {
        Ok(p) => p,
        Err(err) => return JsonRes::err(err),
    };

    let (files, total) = match FileModel::find_list(&ctx.db, &params).await {
        Ok(res) => res,
        Err(err) => return JsonRes::err(err),
    };
    let pager = Pagination {
        total,
        page: params.page.unwrap_or(1),
        page_size: params.page_size.unwrap_or(10),
    };

    let files = files
        .iter()
        .map(|file| {
            let mut file = UploadFileRes::from(file.clone());
            file.url = provider.public_url(&file.uri);
            file
        })
        .collect::<Vec<UploadFileRes>>();

    JsonRes::from((files, pager, "files"))
}

#[debug_handler]
pub async fn add(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateFileReqParams>,
) -> JsonRes<i64> {
    let res = FileModel::upsert(&ctx.db, &params).await;

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
    let settings = get_settings(&ctx);
    let id = params.file_id.unwrap_or(0);

    if id <= 0 {
        return JsonRes::err(format!("数据不存在,id: {id}"));
    }

    let file = match FileModel::find_by_id(&ctx.db, id).await {
        Ok(file) => file,
        Err(err) => return JsonRes::err(err),
    };

    let provider = match StorageProviderFactory::for_driver(&settings.storage, &file.storage) {
        Ok(p) => p,
        Err(err) => return JsonRes::err(err.to_string()),
    };

    if let Err(err) = provider.delete(&file.uri).await {
        return JsonRes::err(format!("删除存储文件失败: {err}"));
    }

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
        }
    };
    let provider = match get_provider(&ctx) {
        Ok(p) => p,
        Err(err) => return JsonRes::err(err),
    };

    let mut _file_name = String::from("");
    let mut file_size = 0;
    let mut file_type = String::from("");
    let mut storage = String::from("local");
    let mut uri = String::from("");
    let mut link = String::from("");
    let mut file_req_params = CreateFileReqParams::default();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();

        if name.as_str().eq("name") {
            _file_name = field.text().await.unwrap_or("".to_string());
        } else if name.as_str().eq("size") {
            let size_str = field.text().await.unwrap_or("".to_string());
            file_size = size_str.parse::<i32>().unwrap_or(0);
        } else if name.as_str().eq("type") {
            file_type = field.text().await.unwrap_or("".to_string());
        } else if name.as_str().eq("storage") {
            storage = field.text().await.unwrap_or("local".to_string());

            if !settings.storage.driver.as_str().eq(storage.as_str()) {
                return JsonRes::err(format!("指定 Storage 配置参数不匹配： {storage}"));
            }
        } else if name.as_str().eq("link") {
            link = field.text().await.unwrap_or("".to_string());
        } else if name.as_str().eq("file") {
            _file_name = if let Some(filename) = field.file_name() {
                filename.to_string()
            } else {
                continue;
            };

            let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
            let mut body_reader = StreamReader::new(body_with_io_error);
            let mut data = Vec::new();

            if let Err(err) = body_reader.read_to_end(&mut data).await {
                return JsonRes::err(err.to_string());
            }

            file_req_params.filename = Some(String::from(_file_name.as_str()));
            file_req_params.storage = Some(String::from(storage.as_str()));
            file_req_params.mime = Some(String::from(file_type.as_str()));
            file_req_params.link = Some(String::from(link.as_str()));
            file_req_params.size = Some(file_size);

            uri = match provider.store(data.into(), &file_req_params).await {
                Ok(res) => res,
                Err(err) => {
                    return JsonRes::err(err.to_string());
                }
            };
        }
    }

    if uri.is_empty() {
        return JsonRes::err(String::from("未上传文件"));
    }

    file_req_params.uri = Some(String::from(uri.as_str()));
    let res = match FileModel::create(&ctx.db, &file_req_params).await {
        Ok(res) => res,
        Err(err) => {
            if let Err(delete_err) = provider.delete(&uri).await {
                tracing::warn!("回滚删除存储文件失败: {delete_err}");
            }
            return JsonRes::err(err.to_string());
        }
    };
    let res_file = match FileModel::find_by_id(&ctx.db, res).await {
        Ok(res) => res,
        Err(err) => {
            return JsonRes::err(err.to_string());
        }
    };

    let mut res = UploadFileRes::from(res_file);
    res.url = provider.public_url(&uri);

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
