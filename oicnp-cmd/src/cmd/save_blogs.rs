use crate::cmd::truncate_all_tables;
use crate::constants::CATEGORIES;
use crate::{models::Blog, Cli};
use oicnp_core::{
    establish_connection,
    models::{NewNode, NewCategory, Node, NewTag},
    prelude::{
        anyhow::{anyhow, Result},
        chrono::NaiveDateTime,
        serde_json,
    },
    services::{
        find_category_by_vid, save_node, save_node_content, save_node_categories_map,
        save_categories, save_tags, find_tag_by_vid, save_node_tags_map,
    },
    typings::{BodyFormat, NodeBundle},
    DatabaseConnection, DbConn, DB,
};
use oicnp_core::utils::generate_slug;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub async fn save_blogs(cli: &Cli) {
    let all_blogs = get_all_blogs(&cli.dist_file);
    let db = DB.get_or_init(establish_connection).await;
    // 清空历史数据
    truncate_all_tables(db).await;

    if let Err(err) = save_taxonomies_data(db).await {
        println!("Taxonomies data save failed {}", err);
    }

     if let Err(err) = save_tags_data(db, all_blogs.as_slice()).await {
         println!("Tags data save failed {}", err);
     }

    let mut date = "";
    if let Some(blog) = all_blogs.get(0) {
        date = blog.date.as_str();
    }

    let mut date_index = 0;
    let mut _i = 0;
    for blog in &all_blogs {
        if !date.eq(blog.date.as_str()) {
            date_index = 0;
            date = blog.date.as_str();
        }

        date_index += 1;
        let blog_res = save_blog(db, blog, date_index).await;

        match blog_res {
            Ok(node) => {
                let body = blog.content.as_ref().unwrap().as_str();
                let res = save_node_content(
                    db,
                    node.nid,
                    body,
                    BodyFormat::Markdown,
                    blog.excerpt.as_str(),
                )
                .await;

                if let Err(err) = res {
                    println!("Save node content failed: {}", err);
                }

                if let Ok(cat_data) = find_category_by_vid(db, blog.category.as_str()).await {
                    if let Err(res) = save_node_categories_map(
                        db,
                        node.bundle.as_str(),
                        node.nid,
                        cat_data.cat_id,
                    )
                    .await
                    {
                        println!("Save Node_taxonomies_map failed: {}", res);
                    }
                }

                save_node_tags_map_data(
                    db,
                    &node,
                    blog.tags.as_slice()
                ).await;

                _i += 1;
            }
            Err(err) => {
                println!("Restore error: {}", err.to_string());
            }
        }
    }
    println!("Restore completed with {} data", _i);
}

async fn save_blog(db: &DatabaseConnection, blog: &Blog, index: i32) -> Result<Node, String> {
    let bundle = NodeBundle::Article;
    let mut rng = thread_rng();
    let hour = format!("{:02}", index);
    let minute = format!("{:02}", rng.gen_range(0..59));
    let second = format!("{:02}", rng.gen_range(0..59));
    let data_str = format!("{} {}:{}:{}", &blog.date, hour, minute, second);
    let date = NaiveDateTime::parse_from_str(&data_str, "%Y-%m-%d %H:%M:%S").unwrap();
    let node = NewNode {
        vid: String::from(&blog.slug),
        bundle: bundle.to_string(),
        title: String::from(&blog.title),
        deleted: false,
        created_by: 1,
        updated_by: 1,
        created_at: date,
        updated_at: date,
        published_at: Some(date),
    };
    let res = save_node(db, &node, &bundle).await;

    match res {
        Ok(node) => Ok(node),
        Err(_err) => Err(format!("Node save failed: {}", &blog.slug)),
    }
}

/// 从JSON文件读取数据
fn get_all_blogs(dist_file: &str) -> Vec<Blog> {
    let mut file = File::open(dist_file).expect("Dist file read failed");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Content read err");

    let blogs: Vec<Blog> = match serde_json::from_str(&contents) {
        Ok(blogs) => blogs,
        _ => vec![],
    };
    blogs
}

async fn save_taxonomies_data(db: &DbConn) -> Result<String> {
    let categories = &CATEGORIES;

    //let mut err = String::from("");
    // 要存储的父级数据
    let mut parent_taxonomies: Vec<NewCategory> = Vec::new();

    // 先把第一父级保存
    for item in categories.iter() {
        if item.parent.eq("") {
            parent_taxonomies.push(NewCategory {
                vid: item.vid.to_string(),
                pid: 0,
                name: item.name.to_string(),
                desc: "".to_string(),
                desc_format: "".to_string(),
                weight: item.weight,
            });
        }
    }

    match save_categories(db, &parent_taxonomies).await {
        Ok(_data) => {}
        Err(err) => {
            println!("Save parent taxonmies failed {:?}", err);
        }
    };

    let mut new_taxonomies: Vec<NewCategory> = Vec::new();
    // 缓存父级 tid Map{vid: tid}
    let mut parent_taxonomies: HashMap<String, i64> = HashMap::new();

    for item in categories.iter() {
        // 跳过已保存的父级
        if item.parent.eq("") {
            continue;
        }
        let mut pid: i64 = 0;

        // 先获取缓存数据
        if let Some(data) = parent_taxonomies.get(item.parent) {
            pid = *data;
        }

        // 缓存数据不存在 且 parent不为空
        if pid == 0 && !item.parent.eq("") {
            if let Ok(res) = find_category_by_vid(db, item.parent).await {
                pid = res.cat_pid;
                parent_taxonomies.insert(item.parent.to_string(), res.cat_id);
            }
        }

        new_taxonomies.push(NewCategory {
            vid: item.vid.to_string(),
            pid,
            name: item.name.to_string(),
            desc: "".to_string(),
            desc_format: "".to_string(),
            weight: item.weight,
        });
    }

    match save_categories(db, &new_taxonomies).await {
        Err(err) => Err(anyhow!(err)),
        _ => Ok("success".to_string()),
    }
}

/// 存储标签数据
async fn save_tags_data(db: &DbConn, all_blogs: &[Blog]) -> Result<String> {
    let mut all_tags: Vec<String> = Vec::new();
    // 收集所有标签
    for item in all_blogs.iter() {
        for tag_str in item.tags.iter() {
            if !all_tags.contains(tag_str) {
                all_tags.push(String::from(tag_str));
            }
        }
    }

    let new_tags = all_tags.iter().map(|item| {
        let (_date, slug) = generate_slug(item);

        return NewTag {
            vid: slug,
            name: String::from(item),
            weight: 0,
            count: 0,
        };
    }).collect::<Vec<NewTag>>();

    match save_tags(db, new_tags.as_slice()).await {
        Err(err) => Err(anyhow!(err)),
        _ => Ok("success".to_string()),
    }
}

// 保存node tag 关联关系
async fn save_node_tags_map_data(db: &DbConn, node: &Node, blog_tags: &[String]) {
    for item in blog_tags.iter() {
        let (_date, vid) = generate_slug(item.as_str());

        match find_tag_by_vid(db, vid.as_str()).await {
            Ok(res) => {
                if let Err(res) = save_node_tags_map(
                    db,
                    node.bundle.as_str(),
                    node.nid,
                    res.tag_id,
                ).await {
                    println!("Save Node_taxonomies_map failed: {}", res);
                }
            },
            Err(err) => {
                println!("find tab err: {:?}", err);
            }
        }
    }
}

