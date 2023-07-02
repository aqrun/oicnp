use crate::{Cli, models::{Blog, Category}};
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use oicnp_core::{DatabaseConnection, DbConn, DB, establish_connection,
    prelude::{
        anyhow::{Result, anyhow},
        chrono::{NaiveDateTime},
        serde_json,
        sea_orm_migration::prelude::*,
    },
    typings::{
        BodyFormat, NodeBundle,
    },
    services::{
        save_node_content, save_node, save_taxonomies, save_node_taxonomies_map,
        find_taxonomy_by_vid,
    },
    models::{
        NewNode, Node, NewTaxonomy, NodeTaxonomiesMap,
    },
};
use rand::{Rng, thread_rng};
use crate::constants::{init_categories, CATEGORIES};
use migration::types as tables;
use crate::cmd::truncate_all_tables;
use std::collections::HashMap;
use oicnp_core::models::Taxonomies;
use oicnp_core::prelude::sea_orm::ColIdx;

pub async fn save_blogs(cli: &Cli) {
    let all_blogs = get_all_blogs(&cli.dist_file);
    let db = DB.get_or_init(establish_connection).await;

    truncate_all_tables(db).await;

    if let Err(err) = save_taxonomies_data(db).await {
        println!("Taxonomies data save failed {}", err);
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
                    node.nid.as_str(),
                    body,
                    BodyFormat::Markdown,
                    blog.excerpt.as_str()
                ).await;

                if let Err(err) = res {
                    println!("Save node content failed: {}", err);
                }

                if let Ok(cat_data) = find_taxonomy_by_vid(db, blog.category.as_str())
                    .await {
                    if let Err(res) = save_node_taxonomies_map(
                        db,
                        node.bundle.as_str(),
                        node.nid.as_str(),
                        cat_data.tid.as_str()
                    ).await {
                        println!("Save Node_taxonomies_map failed: {}", res);
                    }
                }

                _i += 1;
            },
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
    let date = NaiveDateTime::parse_from_str(
        &data_str, "%Y-%m-%d %H:%M:%S"
    ).unwrap();
    let node = NewNode {
        vid: String::from(&blog.slug),
        bundle: bundle.to_string(),
        title: String::from(&blog.title),
        deleted: false,
        created_by: "1".to_string(),
        updated_by: "1".to_string(),
        created_at: date,
        updated_at: date,
        published_at: Some(date),
    };
    let res = save_node(db, &node, &bundle).await;

    match res {
        Ok(node) => Ok(node),
        Err(err) => Err(format!("Node save failed: {}", &blog.slug))
    }
}

/// 从JSON文件读取数据
fn get_all_blogs(dist_file: &str) -> Vec<Blog> {
    let mut file = File::open(dist_file)
        .expect("Dist file read failed");
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
    let categories = CATEGORIES.get_or_init(init_categories);
    let mut err = String::from("");
    // 要存储的父级数据
    let mut parent_taxonomies: Vec<NewTaxonomy> = Vec::new();

    // 先把第一父级保存
    for item in categories.iter() {
        if item.parent.eq("") {
            parent_taxonomies.push(NewTaxonomy {
                vid: item.name.to_string(),
                pid: item.parent.to_string(),
                name: item.name.to_string(),
                description: "".to_string(),
                description_format: "".to_string(),
                weight: item.weight,
            });
        }
    }

    match save_taxonomies(db, &parent_taxonomies).await {
        Ok(_data) => {},
        Err(err) => {
            println!("Save parent taxonmies failed {:?}", err);
        }
    };

    let mut new_taxonomies: Vec<NewTaxonomy> = Vec::new();
    // 缓存父级 tid Map{vid: tid}
    let mut parent_taxonomies: HashMap<String, String> = HashMap::new();

    for item in categories.iter() {
        // 跳过已保存的父级
        if item.parent.eq("") {
            continue;
        }
        let mut pid = "".to_string();

        // 先获取缓存数据
        if let Some(data) = parent_taxonomies.get(item.parent) {
            pid = data.to_string();
        }

        // 缓存数据不存在 且 parent不为空
        if pid.eq("") && !item.parent.eq("") {
            if let Ok(res) = find_taxonomy_by_vid(db, item.parent).await {
                pid = String::from(&res.tid);
                parent_taxonomies.insert(item.parent.to_string(), String::from(&res.tid));
            }
        }

        new_taxonomies.push(NewTaxonomy {
            vid: item.name.to_string(),
            pid,
            name: item.name.to_string(),
            description: "".to_string(),
            description_format: "".to_string(),
            weight: item.weight,
        });
    }

    match save_taxonomies(db, &new_taxonomies).await {
        Err(err) => {
            Err(anyhow!(err))
        },
        _ => {
            Ok("success".to_string())
        },
    }
}
