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
    uuid,
};
use futures::TryStreamExt;
use std::io;
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;
use chrono::Utc;

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
    let mut storage = String::from("");
    let mut url = String::from("");

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
            storage = field.text().await.unwrap_or("".to_string());
        } else if name.as_str().eq("file") {
            let filename = if let Some(filename) = field.file_name() {
                filename.to_string()
            } else {
                continue;
            };

            let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));

            let body_reader = StreamReader::new(body_with_io_error);

            futures::pin_mut!(body_reader);

            // 按日期存储的路径
            let date_path = Utc::now().format("%Y/%m").to_string();
            let ext = filename.as_str().split(".").last().unwrap_or("");
            let real_file_name = format!("{}.{}", uuid!(), ext);
            // 本地存储
            if storage_cfg.driver.as_str().eq("local") {
                let file_path = format!("{}/{}", storage_cfg.path, date_path.as_str());
                tokio::fs::create_dir_all(file_path.clone()).await.unwrap();
                // Create the file. `File` implements `AsyncWrite`.
                let path = std::path::Path::new(file_path.as_str()).join(real_file_name.as_str());
                let mut file = BufWriter::new(File::create(path.clone()).await.unwrap());

                // Copy the body into the file.
                tokio::io::copy(&mut body_reader, &mut file).await.unwrap();
                url = format!("{}/{}/{}", storage_cfg.uri, date_path.as_str(), real_file_name.as_str());
            } else if storage_cfg.driver.as_str().eq("oss") {

            }
        }
    }

    let res = UploadFileRes {
        id: 0,
        name: file_name,
        size: file_size as i64,
        file_type: file_type,
        url,
        ..Default::default()
    };
    
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
