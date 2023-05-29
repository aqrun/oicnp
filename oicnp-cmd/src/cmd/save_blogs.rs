use crate::{Cli, models::{Blog, Category}};
use std::fs::File;
use std::io::prelude::*;
use chrono::{NaiveDateTime};
use oicnp_core::{DatabaseConnection, DB, establish_connection,
    typings::{
        BodyFormat, NodeBundle,
    },
    services::{
        save_node_content, save_node, save_taxonomy, save_node_taxonomies_map,
    },
    models::{
        NewNode, Node, NewTaxonomy, NodeTaxonomiesMap,
    },
};
use rand::{Rng, thread_rng};
use anyhow::{Result, anyhow};

pub async fn save_blogs(cli: &Cli) {
    let all_blogs = get_all_blogs(&cli.dist_file);
    let db = DB.get_or_init(establish_connection).await;

    let categories = vec![
        Category { name: "backend", dir: "blog/backend/_posts" },
        Category { name: "frontend", dir: "blog/frontend/_posts" },
        Category { name: "rust", dir: "blog/rust/_posts" },
        Category { name: "server", dir: "blog/server/_posts" },
        Category { name: "diary", dir: "blog/diary/_posts" },
    ];

    let mut _i = 0;
    for blog in &all_blogs {
        let blog_res = save_blog(db, blog).await;

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

                let cat = blog.category.as_str();
                let new_taxonomy = NewTaxonomy {
                    vid: String::from(cat),
                    pid: "".to_string(),
                    name: String::from(cat),
                    description: "".to_string(),
                    description_format: "".to_string(),
                    weight: 0,
                };
                let res = save_taxonomy(db, &new_taxonomy)
                    .await;

                match res {
                    Ok(taxonomies) => {
                        let res = save_node_taxonomies_map(
                            db, 
                            node.bundle.as_str(),
                            node.nid.as_str(), 
                            taxonomies.tid.as_str()
                        ).await;

                        if let Err(err) = res {
                            println!("Save Node_taxonomies_map failed: {}", err);
                        }
                    },
                    Err(err) => {
                        println!("Save taxonomies failed: {}", err);
                    }
                };


                _i += 1;
            },
            Err(err) => {
                println!("Restore error: {}", err.to_string());
            }
        }
    }
    println!("Restore completed with {} data", _i);
}

async fn save_blog(db: &DatabaseConnection, blog: &Blog) -> Result<Node, String> {
    let bundle = NodeBundle::Article;
    let mut rng = thread_rng();
    let hour = format!("{:02}", rng.gen_range(0..23));
    let minute = format!("{:02}", rng.gen_range(0..59));
    let second = format!("{:02}", rng.gen_range(0..59));
    let data_str = format!("{} {}:{}:{}", &blog.date, hour, minute, second);
    let date = NaiveDateTime::parse_from_str(
        &data_str, "%Y-%m-%d %H:%M:%S"
    ).unwrap();
    let node = NewNode {
        vid: String::from(&blog.slug),
        uid: "1".to_string(),
        bundle: bundle.to_string(),
        title: String::from(&blog.title),
        deleted: false,
        created_by: "1".to_string(),
        updated_by: "1".to_string(),
        created_at: date,
        updated_at: date,
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