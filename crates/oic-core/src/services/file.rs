use loco_rs::prelude::*;
use crate::{
    uuid,
    services::settings::StorageSettings,
    models::{files::CreateFileReqParams},
};
use tokio::{fs::File, io::BufWriter};
use chrono::Utc;


///
/// 存储文件到本地
///
pub async fn store_file_local(
    mut body_reader: impl tokio::io::AsyncRead + Unpin,
    storage_cfg: &StorageSettings,
    params: &CreateFileReqParams,
) -> Result<String, Box<dyn std::error::Error>> {
    let filename = match &params.filename {
        Some(filename) => String::from(filename),
        None => String::from(""),
    };

    // 按日期存储的路径
    let date_path = Utc::now().format("%Y/%m").to_string();
    let ext = filename.split('.').last().unwrap_or("");
    let real_file_name = format!("{}.{}", uuid!(), ext);

    let file_path = format!("{}/{}", storage_cfg.path, date_path);
    tokio::fs::create_dir_all(file_path.clone()).await?;
    
    let path = std::path::Path::new(file_path.as_str()).join(real_file_name.as_str());
    let mut file = BufWriter::new(File::create(path.clone()).await?);

    // Copy the body into the file.
    tokio::io::copy(&mut body_reader, &mut file).await?;
    
    let uri = format!("{}/{}", date_path.as_str(), real_file_name.as_str());

    Ok(uri)
}

/// 存储文件到OSS
pub async fn store_file_oss(
    mut body_reader: impl tokio::io::AsyncRead + Unpin,
    storage_cfg: &StorageSettings,
    params: &CreateFileReqParams,
) -> Result<String, Box<dyn std::error::Error>> {
    let filename = match &params.filename {
        Some(filename) => String::from(filename),
        None => String::from(""),
    };

    // 按日期存储的路径
    let date_path = Utc::now().format("%Y/%m").to_string();
    let ext = filename.split('.').last().unwrap_or("");
    let real_file_name = format!("{}.{}", uuid!(), ext);

    let file_path = format!("{}/{}", storage_cfg.path, date_path);
    
    let uri = format!("{}/{}", date_path.as_str(), real_file_name.as_str());
    Ok(uri)
}