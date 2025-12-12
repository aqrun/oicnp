use crate::constants::{CATEGORIES, VID_READING};
use crate::models::{Blog, BlogMatter, Category, MatterTaxonomy};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use loco_rs::prelude::ModelError;
use oic_core::{
    AppContext,
    utils::{generate_slug, is_valid_matter_content},
    entities::prelude::*,
};
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use serde_json;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

///
/// 保存博客
/// 
pub async fn save_blogs(ctx: &AppContext, blogs_file: &str) -> Result<()> {
    log::info!(
        "\n\n\n[OICP] 保存博客开始 at {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    let blogs = read_blogs(blogs_file)?;

    // 全部数据计数
    let mut success_amount = 0;
    let mut failed_amount = 0;
    // 跳过数据计数
    let mut jump_amount = 0;
    let mut jump_messages: Vec<String> = vec![];
    let mut failed_messages: Vec<String> = vec![];

    for blog in blogs {
        let params = blog.to_create_node_req_params();
        let _ = match NodeModel::upsert_by_vid(&ctx.db, &params).await {
            Ok(id) => {
                success_amount += 1;
                // log::info!("[OICNP] Blog 保存成功: {} <{}>", blog.slug.as_str(), blog.title.as_str());
                id
            },
            Err(err) => {
                failed_amount += 1;

                match err {
                    ModelError::Message(message) => {
                        jump_amount += 1;
                        jump_messages.push(String::from(message.as_str()));
                        log::warn!("[OICNP] Blog 添加失败, {:?}", message);
                        continue;
                    },
                    _ => {
                        failed_messages.push(format!("{} <{}>", blog.slug.as_str(), blog.title.as_str()));
                        return Err(anyhow!("[OICNP] Blog 添加失败, {:?}", err));
                    }
                }
            }
        };
    }

    let res_content = format!(r#"
全部数量：{}
失败数量：{}
跳过数量：{}

跳过数据：
{}

失败数据：
{}
"#, success_amount, failed_amount, jump_amount, jump_messages.join("\n"), failed_messages.join("\n"));

    println!("{}", res_content);

    log::info!(
        "\n\n\n[OICP] 保存博客结束 at {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    Ok(())
}

///
/// 收集所有博客数据
/// 
pub async fn find_all_blog(_format: &str, blog_base: &str, dist_file: &str) -> Result<()> {
    log::info!(
        "\n\n\n[OICP] Blog handle start at {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    // let db = DB.get_or_init(establish_connection).await;
    let categories = &CATEGORIES;

    let all_blogs = find_all_blogs(categories, blog_base)?;
    
    // println!("all_blogs {}", serde_json::to_string(&all_blogs[0..2]).unwrap());
    match save_to_file(&all_blogs, dist_file) {
        Err(err) => {
            log::error!("[OICNP] Save to file failed, {:?}", err);
        }
        _ => {
            log::info!(
                "\n[OICNP] Blog save completed! total: {}",
                &all_blogs.capacity()
            );
        }
    }
    log::info!(
        "\n[OICNP] Blog handle end at: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    Ok(())
}

/// 保存到文件
fn save_to_file(all_blogs: &[Blog], dist_file: &str) -> Result<String> {
    let path = PathBuf::from(dist_file);
    fs::write(path, serde_json::to_string(all_blogs).unwrap())?;
    Ok(String::from(""))
}

fn generate_blog(
    file_name: String,
    content: String,
    file_path: String,
    category: String,
) -> Result<Blog> {
    if !is_valid_matter_content(&content) {
        log::warn!(
            "[OICNP] Not valid matter content: {}{}",
            &file_path, &file_name
        );
        return Err(anyhow!("[OICNP]Not valid matter content: {}", &file_name));
    }

    let (date, slug) = generate_slug(&file_name);
    // println!("Matter parse: {}", &file_name);
    let matter = Matter::<YAML>::new();
    let res: ParsedEntity<BlogMatter> = match matter.parse(&content) {
        Ok(data) => data,
        Err(err) => {
            println!("[OICNP]Matter parse failed: {}{}", &file_path, &file_name);
            return Err(anyhow!(
                "[OICNP]Matter parse failed: {}{}",
                &file_path,
                &file_name
            ));
        }
    };
    // println!("{:?}", res.data);
    let data = match res.data {
        Some(data) => data,
        None => BlogMatter::default(),
    };
    let con = res.content;

    // let layout = &data["layout"].as_string().unwrap();
    let title = data.title.unwrap_or(String::from(""));
    let default_meta_taxonomy = MatterTaxonomy {
        categories: vec![],
        tags: vec![],
    };
    let meta_taxonomies = data.taxonomies.unwrap_or(default_meta_taxonomy);
    let tags = meta_taxonomies.tags;
    let excerpt = data.description.unwrap_or(String::from(""));

    let blog = Blog {
        slug,
        date,
        file: String::from(&file_name),
        file_path,
        title,
        tags,
        excerpt,
        category,
        content: Some(con),
    };
    Ok(blog)
}

/// 获取所有 blog 数据
fn find_all_blogs(categories: &Vec<Category>, blog_base: &str) -> Result<Vec<Blog>> {
    let base = PathBuf::from(blog_base);

    if !base.exists() {
        return Err(anyhow!("目录不存在: {}", blog_base));
    }

    let mut all_blogs: Vec<Blog> = vec![];

    // 内容分类
    let article_categories = categories
        .iter()
        .filter(|item| {
            item.parent.eq("cms") && !item.vid.eq(VID_READING)
        })
        .collect::<Vec<&Category>>();

    for item in article_categories.iter() {
        if item.dir.eq("") {
            continue;
        }
        let mut dir = base.clone();
        dir.push(item.dir);

        let str_dir = String::from(dir.to_str().unwrap());
        let entries = fs::read_dir(dir).expect(&format!("Read dir failed: {}", &str_dir));

        for m in entries {
            let entry = m.unwrap();

            // println!("entyr--- {:?}, {:?}", &entry.path(), &entry.path().ends_with("_index.md"));
            // _index.md 文件不需要处理
            let is_index_file = (&entry).path().ends_with("_index.md");
            if is_index_file {
                continue;
            }

            if let Ok(mut file) = fs::File::open(entry.path()) {
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .expect("Read content failed");

                if let Ok(blog) = generate_blog(
                    entry.file_name().into_string().expect("Invalid string"),
                    content,
                    String::from(entry.path().to_str().expect("Invalid str")),
                    String::from(item.vid),
                ) {
                    all_blogs.push(blog);
                }
            }
            // println!("{}", content)
        }
    }

    Ok(all_blogs)
}

///
/// 从指定文件读取博客数据
/// 
pub fn read_blogs(blogs_file: &str) -> Result<Vec<Blog>> {
    let message: String = std::fs::read_to_string(blogs_file).expect(&format!("文件不存在: {}", blogs_file));
    let blogs = serde_json::from_str::<Vec<Blog>>(message.as_str())?;
    Ok(blogs)
}