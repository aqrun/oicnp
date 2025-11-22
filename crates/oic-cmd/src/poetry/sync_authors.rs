use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use oic_core::{
    entities::poetry::*,
    uuid,
    models::poetry::*,
};
use crate::utils::format_duration;
use super::{db_conn, DB};


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct AuthorLoader {
    pub path: String,
    pub dynasty: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct AuthorData {
    pub description: Option<String>,
    pub name: String,
    pub short_description: Option<String>,
}

///
/// 同步诗词作者数据
/// 
/// 1 读取 loaders/authors.json 文件
/// 2 遍历文件，读取文件内容
/// 3 解析文件内容，插入数据库
/// 
pub async fn sync_authors_data(poetry_dir: &str) -> Result<()> {
    println!("开始同步诗词作者数据");
    // 记录开始时间
    let start_time = std::time::Instant::now();
    let db = DB.get_or_init(db_conn).await;
    let loader_config_file = format!("crates/oic-cmd/src/poetry/loaders/authors.json");
    let loader_content = fs::read_to_string(loader_config_file.as_str())?;
    let loader_data: Vec<AuthorLoader> = serde_json::from_str(&loader_content)?;

    // 记录总数
    let mut count = 0;
    
    for loader in loader_data {
        let loader_path = format!("{}/{}", poetry_dir, loader.path.as_str());
        let loader_content = match fs::read_to_string(loader_path.as_str()) {
            Ok(content) => content,
            Err(e) => {
                println!("读取文件失败: {} - {}", e, loader_path.as_str());
                continue;
            }
        };
        let author_data_list: Vec<AuthorData> = match serde_json::from_str(&loader_content) {
            Ok(data) => data,
            Err(e) => {
                println!("解析文件失败: {} - {}", e, loader_path.as_str());
                continue;
            }
        };

        for data in author_data_list {
            let mut author = CreateAuthorReqParams {
                uuid: Some(uuid!()),
                name: Some(String::from(data.name.as_str())),
                dynasty: Some(String::from(loader.dynasty.as_str())),
                ..Default::default()
            };

            if let Some(n) = &data.description {
                if !n.is_empty() && n.as_str() != "--" {
                    author.description = Some(String::from(n.as_str()));
                }
            }

            if let Some(n) = &data.short_description {
                if !n.is_empty() && n.as_str() != "--" {
                    author.short_description = Some(String::from(n.as_str()));
                }
            }

            let _ =match AuthorModel::upsert(db, &author).await {
                Ok(id) => id,
                Err(e) => {
                    println!("插入数据失败: {} - {}", e, loader_path.as_str());
                    println!("AuthorData: {:?}", data);
                    continue;
                }
            };
            count += 1;
        }
    }
    // 记录结束时间 打印秒级时间
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("同步诗词作者数据完成，耗时: {}, 总数: {}", format_duration(duration), count);
    Ok(())
}