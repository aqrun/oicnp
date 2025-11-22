use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use oic_core::{
    entities::poetry::*,
    uuid,
    models::ModelCrudHandler,
    models::poetry::*,
};
use super::{db_conn, DB};
use crate::utils::format_duration;

/// 测试阶段添加一个变量控制数据插入数量
/// 单个文件只处理前10个数据
pub const TEST_MAX_COUNT: i32 = 0;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct PoetryLoader {
    /// 诗集名
    pub name: String,
    /// 诗集id
    pub id: i32,
    /// 诗集文件或目录路径
    pub path: String,
    /// 排除的文件列表
    pub excludes: Option<Vec<String>>,
    /// 诗集朝代
    pub dynasty: String,
    /// 诗集标签
    pub tags: String,
    /// 诗集数据key
    pub data_key: String,
    /// 诗词作者
    pub author: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct PoetryData {
    /// 作者
    pub author: Option<String>,
    /// 诗词标题
    pub title: Option<String>,
    /// 诗词内容
    pub content: Option<Vec<String>>,
    pub comment: Option<Vec<String>>,
    /// 诗词段落
    pub paragraphs: Option<Vec<String>>,
    /// 诗词段落
    pub para: Option<Vec<String>>,
    /// 词牌名 title为空时 词牌名为title
    pub rhythmic: Option<String>,
    /// 诗词标签列表
    pub tags: Option<Vec<String>>,
    /// 诗词卷数 存入标签
    pub volume: Option<String>,
    /// 诗词序言 说明
    pub prologue: Option<String>,
    pub biography: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct YouMengYingPoetryData {
    /// 诗词内容
    pub content: Option<String>,
    pub comment: Option<Vec<String>>,
}

///
/// 同步五代诗词数据
/// 
/// 1读取loaders/poetry.json文件
/// 2遍历文件，读取文件内容
/// 3解析文件内容，插入数据库
/// 
/// loader.path 指定路径的类型
/// - 如果是文件，直接读取内容
/// - 如果是目录，需要递归遍历子目录，对每个JSON文件调用处理函数
pub async fn sync_poetry_data(poetry_dir: &str) -> Result<()> {
    // 记录开始时间
    let start_time = std::time::Instant::now();
    println!("开始同步诗词数据");
    let loader_config_file = format!("crates/oic-cmd/src/poetry/loaders/poetry.json");
    let loader_content = fs::read_to_string(loader_config_file.as_str())?;
    let loader_data: Vec<PoetryLoader> = serde_json::from_str(&loader_content)?;

    let mut total_count = 0;
    for loader in loader_data {
        let loader_path = Path::new(poetry_dir).join(&loader.path);
        
        // 判断路径是文件还是目录
        if loader_path.is_file() {
            // 类型1: 是文件，直接读取内容处理
            println!("处理文件: {}", loader.path);
            match process_poetry_file(&loader, loader_path.to_str().unwrap()).await {
                Ok(count) => {
                    total_count += count;
                    println!("文件处理完成: {}, 处理数量: {}", loader.path, count);
                },
                Err(e) => {
                    println!("处理文件失败: {} - {}", loader.path, e);
                    continue;
                }
            }
        } else if loader_path.is_dir() {
            // 类型2: 是目录，需要递归遍历子目录
            println!("处理目录: {}", loader.path);
            match find_json_files(&loader_path, &loader.excludes) {
                Ok(json_files) => {
                    let file_count = json_files.len();
                    let mut dir_count = 0;
                    for json_file in json_files {
                        let file_path = json_file.to_str().unwrap();
                        match process_poetry_file(&loader, file_path).await {
                            Ok(count) => {
                                dir_count += count;
                                total_count += count;
                            },
                            Err(e) => {
                                println!("处理文件失败: {} - {}", file_path, e);
                                continue;
                            }
                        }
                    }
                    println!("目录处理完成: {}, 处理文件数: {}, 处理数量: {}", loader.path, file_count, dir_count);
                },
                Err(e) => {
                    println!("遍历目录失败: {} - {}", loader.path, e);
                    continue;
                }
            }
        } else {
            println!("路径不存在或类型不正确: {}", loader.path);
            continue;
        }
    }

    let youmengying_count = sync_youmengying_data(poetry_dir).await?;
    total_count += youmengying_count;

    // 记录结束时间 打印秒级时间
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("同步诗词数据完成，耗时: {}, 总数: {}", format_duration(duration), total_count);
    Ok(())
}

/// 根据data_key获取诗词内容
fn get_poetry_content_by_data_key(item: &PoetryData, data_key: &str) -> Option<Vec<String>> {
    match data_key {
        "content" => {
            if let Some(content) = &item.content {
                Some(content.clone())
            } else {
                None
            }
        },
        "comment" => {
            if let Some(comment) = &item.comment {
                Some(comment.clone())
            } else {
                None
            }
        },
        "paragraphs" => {
            if let Some(paragraphs) = &item.paragraphs {
                Some(paragraphs.clone())
            } else {
                None
            }
        },
        "para" => {
            if let Some(para) = &item.para {
                Some(para.clone())
            } else {
                None
            }
        },
        _ => None
    }
}

/// 递归遍历目录，查找所有JSON文件
fn find_json_files(dir: &Path, excludes: &Option<Vec<String>>) -> Result<Vec<PathBuf>> {
    let mut json_files = Vec::new();
    
    if !dir.exists() || !dir.is_dir() {
        return Ok(json_files);
    }
    
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // 检查是否在排除列表中
        if let Some(excludes) = excludes {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if excludes.iter().any(|ex| file_name.contains(ex)) {
                continue;
            }
        }
        
        if path.is_dir() {
            // 递归遍历子目录
            let mut sub_files = find_json_files(&path, excludes)?;
            json_files.append(&mut sub_files);
        } else if path.is_file() {
            // 检查是否是JSON文件
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    json_files.push(path);
                }
            }
        }
    }
    
    Ok(json_files)
}

/// 处理单个诗词数据项
async fn process_poetry_item(
    loader: &PoetryLoader,
    item: &PoetryData,
    weight: i32,
) -> Result<i32> {
    let db = DB.get_or_init(db_conn).await;
    // loader.dynasty 为 check_by_file_name 时 需要根据文件名判断朝代
    // 文件名是 poet.song 开头是 宋
    // 文件名是 poet.tang 开头是 唐
    // 文件名带 唐诗 的是唐
    let mut dynasty = String::from(loader.dynasty.as_str());

    if loader.dynasty == "check_by_file_name" {
        let file_name = Path::new(&loader.path).file_name().unwrap_or_default().to_str().unwrap_or("");
        if file_name.starts_with("poet.song") {
            dynasty = "宋".to_string();
        } else if file_name.starts_with("poet.tang") {
            dynasty = "唐".to_string();
        } else if file_name.contains("唐诗") {
            dynasty = "唐".to_string();
        }
    }
    // 作者信息有两部分获取
    // 1是当前诗词信息中的author字段
    // 2是loader_data中的author字段
    // 3都没有 author就留空直接保存诗词数据
    let mut author_name = String::from("");
    if let Some(author) = &item.author {
        author_name = String::from(author);
    } else if let Some(author) = &loader.author {
        author_name = String::from(author);
    }

    let author_params = CreateAuthorReqParams {
        name: Some(author_name),
        dynasty: Some(String::from(dynasty.as_str())),
        ..Default::default()
    };
    
    let author_id = match AuthorModel::upsert(db, &author_params).await {
        Ok(id) => id,
        Err(_e) => {
            0
        }
    };

    // 创建诗词参数
    // 1title 为空的 使用 rhythmic字段作为标题
    // 2根据 data_key 指定的字段 获取诗词内容
    // 3 poetry.content 使用上面内容 join("\n")
    let mut poetry_params = CreatePoetryReqParams {
        uuid: Some(uuid!()),
        author_id: Some(author_id as i32),
        dynasty: Some(String::from(dynasty.as_str())),
        weight: Some(weight),
        ..Default::default()
    };

    // 标题处理
    if let Some(n) = &item.title {
        poetry_params.title = Some(String::from(n));
    } else if let Some(rhythmic) = &item.rhythmic {
        poetry_params.title = Some(String::from(rhythmic));
    }

    // 诗词内容 - 根据data_key获取
    if let Some(content_list) = get_poetry_content_by_data_key(item, &loader.data_key) {
        let content = content_list.join("\n");
        poetry_params.content = Some(String::from(content.as_str()));
        poetry_params.word_count = Some(content.len() as i32);
    }

    // 诗词说明
    if let Some(n) = &item.prologue {
        poetry_params.description = Some(String::from(n));
    }

    // 诗词标签
    // 标签也有两部分
    // 1是当前诗词信息中的tags字段
    // 2是loader_data中的tags字段
    if let Some(tags_list) = &item.tags {
        let mut all_tags = tags_list.clone();
        if !loader.tags.is_empty() {
            all_tags.push(String::from(loader.tags.as_str()));
        }
        poetry_params.tags = Some(all_tags.join(","));
    } else if !loader.tags.is_empty() {
        poetry_params.tags = Some(String::from(loader.tags.as_str()));
    }

    if let Some(comment_list) = &item.comment {
        let comment = comment_list.join("\n");
        poetry_params.description = Some(String::from(comment.as_str()));
    }

    match PoetryModel::create(db, &poetry_params).await {
        Ok(_) => {
            // 更新作者作品计数
            AuthorModel::update_count_by_id(db, author_id as i32).await?;
            Ok(1)
        },
        Err(e) => {
            Err(anyhow::anyhow!("插入诗词数据失败: {} - {}", e, loader.path.as_str()))
        }
    }
}

/// 处理单个文件
/// 测试阶段添加一个变量控制数据插入数量
/// 单个文件只处理前10个数据
async fn process_poetry_file(
    loader: &PoetryLoader,
    poetry_file: &str,
) -> Result<usize> {
    let content = fs::read_to_string(poetry_file)?;
    let poetry_list: Vec<PoetryData> = match serde_json::from_str(&content) {
        Ok(x) => x,
        Err(e) => {
            return Err(anyhow::anyhow!("解析文件数据失败: {:?} - {}", poetry_file, e));
        }
    };

    let mut count = 0;
    let mut weight = 0;
    for item in poetry_list {
        if TEST_MAX_COUNT > 0 && count >= TEST_MAX_COUNT as usize {
            break;
        }
        weight += 1;
        match process_poetry_item(loader, &item, weight).await {
            Ok(_) => count += 1,
            Err(e) => {
                println!("处理诗词项失败: {} - {}", e, poetry_file);
                continue;
            }
        }
    }
    
    Ok(count)
}


/// 这个不需要优化先这样处理
/// 幽梦影-张潮文集 单独处理
/// 幽梦影 content 是 string 和上面不统一
pub async fn sync_youmengying_data(poetry_dir: &str) -> Result<usize> {
    let loader_config = r#"
    {
        "name": "幽梦影-张潮文集",
        "id": 6,
        "path": "幽梦影/youmengying.json",
        "dynasty": "清",
        "author": "张潮",
        "tags": "幽梦影",
        "data_key": "content"
    }
    "#;
    let loader: PoetryLoader = serde_json::from_str(&loader_config)?;
    println!("处理目录: {}", loader.path);
    let loader_path = Path::new(poetry_dir).join(&loader.path);
    // 存在 "comment": "" 这种为空的字符串 需要转换为空数组
    let mut content = fs::read_to_string(loader_path.to_str().unwrap())?;
    content = content.replace("\"comment\": \"\"", "\"comment\": []");
    let poetry_list: Vec<YouMengYingPoetryData> = match serde_json::from_str(content.as_str()) {
        Ok(x) => x,
        Err(e) => {
            return Err(anyhow::anyhow!("解析文件数据失败: {:?} - {}", loader_path.to_str().unwrap(), e));
        }
    };

    let mut count = 0;

    for item in poetry_list {
        if TEST_MAX_COUNT > 0 && count >= TEST_MAX_COUNT as usize {
            break;
        }

        let mut poetry_data = PoetryData {
            title: Some(String::from("幽梦影")),
            ..Default::default()
        };
        if let Some(content) = &item.content {
            poetry_data.content = Some(vec![content.clone()]);
        }
        if let Some(comment) = &item.comment {
            poetry_data.comment = Some(comment.clone());
        }

        process_poetry_item(&loader, &poetry_data, 1).await?;
        count += 1;
    }

    println!("幽梦影处理完成: {}, 处理数量: {}", loader.path, count);

    Ok(count)
}


/// 御定全唐詩 需要单独处理 和全唐诗有重复
/// 已有数据直接更新
pub async fn sync_yuding_quan_tang_shi(poetry_dir: &str) -> Result<()> {
    let start_time = std::time::Instant::now();
    let db = DB.get_or_init(db_conn).await;
    let loader_config = r#"
    {
        "name": "御定全唐詩",
        "id": 7,
        "path": "御定全唐詩/json/",
        "dynasty": "唐",
        "tags": "御定全唐诗",
        "data_key": "paragraphs"
    }
    "#;
    let loader: PoetryLoader = serde_json::from_str(&loader_config)?;
    println!("处理目录: {}", loader.path);
    let loader_path = Path::new(poetry_dir).join(&loader.path);
    // 遍历目录，读取所有JSON文件
    let json_files = find_json_files(&loader_path, &None)?;

    let mut create_count = 0;
    let mut update_count = 0;
    
    for json_file in json_files {
        let content = match fs::read_to_string(json_file.to_str().unwrap()) {
            Ok(x) => x,
            Err(e) => {
                return Err(anyhow::anyhow!("读取文件数据失败: {:?} - {}", json_file.to_str().unwrap(), e));
            }
        };

        let poetry_list: Vec<PoetryData> = match serde_json::from_str(&content) {
            Ok(x) => x,
            Err(e) => {
                return Err(anyhow::anyhow!("解析文件数据失败: {:?} - {}", json_file.to_str().unwrap(), e));
            }
        };
        for item in poetry_list {
            let mut create_author_params = CreateAuthorReqParams {
                dynasty: Some(String::from(loader.dynasty.as_str())),
                ..Default::default()
            };

            if let Some(author) = &item.author {
                create_author_params.name = Some(String::from(author));
            }

            let author_id = match AuthorModel::upsert(db, &create_author_params).await {
                Ok(id) => id,
                Err(e) => {
                    log::error!("插入作者数据失败: {:?} - {}", json_file.to_str().unwrap(), e);
                    0
                }
            };

            let mut create_poetry_params = CreatePoetryReqParams {
                author_id: Some(author_id as i32),
                dynasty: Some(String::from(loader.dynasty.as_str())),
                ..Default::default()
            };

            if let Some(title) = &item.title {
                create_poetry_params.title = Some(String::from(title));
            }

            if let Some(content) = &item.paragraphs {
                create_poetry_params.content = Some(content.join("\n"));
            }

            if let Some(prologue) = &item.prologue {
                create_poetry_params.description = Some(String::from(prologue));
            }

            match PoetryModel::upsert(db, &create_poetry_params).await {
                Ok((_, update_or_create)) => {
                    // 是新增数据需要更新作者作品计数
                    if update_or_create == "create" {
                        AuthorModel::update_count_by_id(db, author_id as i32).await?;
                        create_count += 1;
                    } else if update_or_create == "update" {
                        update_count += 1;
                    }
                },
                Err(e) => {
                    log::error!("插入诗词数据失败: {:?} - {}", json_file.to_str().unwrap(), e);
                }
            };
        }
    }

    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("御定全唐詩处理完成: {}, 新增数量: {}, 更新数量: {}, 耗时: {}",
        loader.path,
        create_count,
        update_count,
        format_duration(duration)
    );

    Ok(())
}