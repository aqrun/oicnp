extern crate fast_log;

use std::path::PathBuf;
use std::fs;
use std::io::Read;
use oicnp_core::{G, establish_connection, DB, DatabaseConnection};
use gray_matter::Matter;
use gray_matter::engine::YAML;
use oicnp_api::models::{
    Nodes, NewNode,
};
use oicnp_api::typings::{NodeBundle, BodyFormat};
use oicnp_api::utils::{
    generate_slug,
    is_valid_matter_content,
};
use serde::{Deserialize, Serialize};
use log::{ warn, error, info };
use rand::{Rng, thread_rng};
use fast_log::{
    plugin::{
        file_split::RollingType,
        packer::LogPacker,
    },
    consts::LogSize,
};
use chrono::prelude::*;
use crate::models::{Blog, Category, BlogMatter};
use anyhow::{anyhow, Result};

pub async fn run(format: &str, blog_base: &str, dist_file: &str) {
    fast_log::init(fast_log::Config::new()
        .console()
        .chan_len(Some(100000))
        .file_split(
            "target/logs/",
            LogSize::MB(1),
            RollingType::All,
            LogPacker{}
        )).unwrap();
    
    info!("\n\n\n[OICP] Blog handle start at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    // let db = DB.get_or_init(establish_connection).await;

    let categories = vec![
        Category { name: "backend", dir: "blog/backend/_posts" },
        Category { name: "frontend", dir: "blog/frontend/_posts" },
        Category { name: "rust", dir: "blog/rust/_posts" },
        Category { name: "server", dir: "blog/server/_posts" },
        Category { name: "diary", dir: "blog/diary/_posts" },
    ];

    let all_blogs = find_all_blogs(categories, blog_base);
    // println!("all_blogs {}", serde_json::to_string(&all_blogs[0..2]).unwrap());
    match save_to_file(&all_blogs, dist_file) {
        Err(err) => {
            error!("[OICNP] Save to file failed, {:?}", err);
        }
        _ => {
            info!("\n[OICP] Blog save completed! total: {}", &all_blogs.capacity());
        }
    }
    info!("\n[OICP] Blog handle end at: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
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
    category: String
) -> Result<Blog> {
    if !is_valid_matter_content(&content) {
        warn!("[OICNP] Not valid matter content: {}{}", &file_path, &file_name);
        return Err(anyhow!("[OICNP]Not valid matter content: {}", &file_name));
    }

    let (date, slug) = generate_slug(&file_name);
    // println!("Matter parse: {}", &file_name);
    let matter = Matter::<YAML>::new();
    let res = match matter.parse_with_struct::<BlogMatter>(&content) {
            Some(data) => data,
            _ => {
                println!("[OICNP]Matter parse failed: {}{}", &file_path, &file_name);
                return Err(anyhow!("[OICNP]Matter parse failed: {}{}", &file_path, &file_name));
            }
    };

    let data = res.data;
    let con = res.content;

    // let layout = &data["layout"].as_string().unwrap();
    let title = data.title.unwrap_or(String::from(""));
    let tags = data.tags.unwrap_or(String::from(""));
    let tag_arr = tags.split(" ")
        .map(|item| String::from(item))
        .collect();
    let excerpt = data.excerpt.unwrap_or(String::from(""));

    let blog = Blog {
        slug,
        date,
        file: String::from(&file_name),
        file_path,
        title,
        tags: tag_arr,
        excerpt,
        category,
        content: Some(con),
    };
    Ok(blog)
}

/// 获取所有 blog 数据
fn find_all_blogs(categories: Vec<Category>, blog_base: &str) -> Vec<Blog> {
    let base = PathBuf::from(blog_base);
    let mut all_blogs: Vec<Blog> = vec!();
    let mut index = 0;

    for item in categories {
        let mut dir = base.clone();
        dir.push(item.dir);

        let str_dir = String::from(dir.to_str().unwrap());
        let entries = fs::read_dir(dir).expect(&format!("Read dir failed: {}", &str_dir));

        for m in entries {
            let entry = m.unwrap();
            let mut file = fs::File::open(entry.path()).expect("File read failed");
            let mut content = String::new();
            file.read_to_string(&mut content).expect("Read content failed");

            if let Ok(blog) = generate_blog (
                entry.file_name().into_string().expect("Invalid string"),
                content,
                String::from(entry.path().to_str().expect("Invalid str")),
                String::from(item.name)
            ) {
                all_blogs.push(blog);
            }

            index += 1;
            // println!("{}", content)
        }
    }

    all_blogs
}