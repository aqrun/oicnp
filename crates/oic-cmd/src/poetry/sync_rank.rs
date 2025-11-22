#[allow(dead_code)]
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use oic_core::{
    entities::poetry::*,
    models::ModelCrudHandler,
    models::poetry::*,
};
use super::db::{db_conn, DB};
use crate::utils::format_duration;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct RankData {
    pub author: String,
    pub title: String,
    pub baidu: u64,
    pub so360: u64,
    pub bing: u64,
    pub bing_en: u64,
    pub google: u64,
    pub dynasty: String,
}

/**
 * 处理 /rank 目录的搜索排名，只取 baidu 搜索排名同步到诗词 hot_weight
 * 
 * 1读取 poem_rank_folder 目录下的所有文件
 * 2遍历文件，读取文件内容
 * 3解析文件内容，使用 author 和 title 同是匹配诗词表 将baidu数据更新到诗词表的 hot_weight 字段
 */
pub async fn sync_rank_data(poetry_dir: &str) -> Result<()> {
    // 记录开始时间
    let start_time = std::time::Instant::now();
    println!("开始同步诗词搜索排名数据");
    let db = DB.get_or_init(db_conn).await;

    // 缓存全部作者信息
    /*
    let (author_list, _) = match AuthorModel::find_list(db, &AuthorFilters::default()).await {
        Ok(data) => data,
        Err(e) => {
            return Err(anyhow::anyhow!("获取作者列表失败: {}", e));
        }
    };
    */

    let poem_rank_folder = format!("{}/rank/poet", poetry_dir);

    let poem_rank_files = match fs::read_dir(poem_rank_folder.as_str()) {
        Ok(files) => files,
        Err(e) => {
            return Err(anyhow::anyhow!("读取诗词搜索排名文件失败: {} - {}", e, poem_rank_folder));
        }
    };

    let mut count = 0;

    for file in poem_rank_files {
        let file_path = file.unwrap().path();
        let file_content = fs::read_to_string(file_path.as_path())?;
        // 根据文件名获取朝代
        let dynasty = match file_path.file_name().unwrap_or_default().to_str() {
            Some(x) => {
                if x.starts_with("poet.song") {
                    String::from("宋")
                } else if x.starts_with("poet.tang") {
                    String::from("唐")
                } else {
                    String::from("")
                }
            },
            None => String::from(""),
        };
        let rank_data_list: Vec<RankData> = match serde_json::from_str(&file_content) {
            Ok(data) => data,
            Err(e) => {
                return Err(anyhow::anyhow!("解析诗词搜索排名文件失败: {} - {}", e, file_path.display()));
            }
        };
        // 补充朝代信息
        let rank_data_list = rank_data_list.iter().map(|x| {
            RankData {
                author: x.author.clone(),
                title: x.title.clone(),
                baidu: x.baidu,
                so360: x.so360,
                bing: x.bing,
                bing_en: x.bing_en,
                google: x.google,
                dynasty: dynasty.clone(),
            }
        }).collect::<Vec<RankData>>();
        for rank_data in rank_data_list {
            count += 1;

            let poetry_filters = PoetryFilters {
                title: Some(String::from(rank_data.title.as_str())),
                // author_id: Some(author.id),
                ..Default::default()
            };

            match PoetryModel::update_hot_weight(db, poetry_filters, rank_data.baidu as i16).await {
                Ok(_) => (),
                Err(e) => {
                    log::error!("更新诗词搜索排名失败: {} - {}", e, file_path.display());
                    continue;
                }
            };

            /*
            // 作者可能存在多个同名
            let authors = author_list.iter().filter(|x| {
                if !rank_data.dynasty.is_empty()
                    && x.name.eq(&rank_data.author)
                    && x.dynasty == rank_data.dynasty
                {
                    return true;
                } else if x.name.eq(&rank_data.author) {
                    return true;
                }

                return false;
            }).collect::<Vec<&AuthorModel>>();

            for author in authors {
                let poetry_filters = PoetryFilters {
                    title: Some(String::from(rank_data.title.as_str())),
                    // author_id: Some(author.id),
                    ..Default::default()
                };

                match PoetryModel::update_hot_weight(db, poetry_filters, rank_data.baidu as i16).await {
                    Ok(_) => (),
                    Err(e) => {
                        log::error!("更新诗词搜索排名失败: {} - {}", e, file_path.display());
                        continue;
                    }
                };
            }
            */
        }
    }

    // 记录结束时间 打印秒级时间
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("同步诗词搜索排名数据完成，耗时: {}, 总数: {}", format_duration(duration), count);
    Ok(())
}