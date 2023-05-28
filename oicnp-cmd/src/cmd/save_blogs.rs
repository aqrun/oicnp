use crate::{Cli, models::{Blog}};
use std::fs::File;
use std::io::prelude::*;
use oicnp_api::{
    models::{Nodes, NewNode},
    typings::{NodeBundle},
    services::{save_node},
};
use chrono::{NaiveDateTime};
use oicnp_core::{DatabaseConnection};
use rand::{Rng, thread_rng};
use anyhow::{Result, anyhow};

pub async fn save_blogs(cli: &Cli) {
    let all_blogs = get_all_blogs(&cli.dist_file);

    println!("blog read: {:?}", all_blogs);
}

async fn save_blog(db: &DatabaseConnection, blog: &Blog) -> Result<Nodes, String> {
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
        uid: 1,
        bundle: bundle.to_string(),
        title: String::from(&blog.title),
        deleted: false,
        created_by: 1,
        updated_by: 1,
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