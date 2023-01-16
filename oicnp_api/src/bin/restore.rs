/// 历史数据恢复
///
extern crate fast_log;

use std::path::PathBuf;
use std::fs;
use std::io::Read;
use oicnp_api::services::G;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use oicnp_api::dbs::{init_rbatis};
use std::sync::Arc;
use oicnp_api::models::{
    Nodes, NewNode,
};
use oicnp_api::typings::{NodeBundle, BodyFormat};
use oicnp_api::services::{
    save_category,
    save_tags,
    save_node_content,
    save_node,
};
use oicnp_api::utils::{
    generate_slug,
    is_valid_matter_content,
};
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use log::{ warn };
use rand::{Rng, thread_rng};
use fast_log::{
    plugin::{
        file_split::RollingType,
        packer::LogPacker,
    },
    consts::LogSize,
};

#[derive(Debug)]
struct Category<'a> {
    name: &'a str,
    dir: &'a str,
}

#[derive(Debug)]
struct Blog {
    slug: String,
    date: String,
    file: String,
    file_path: String,
    title: String,
    tags: Vec<String>,
    excerpt: String,
    category: String,
    content: Option<String>,
}

impl Blog {
    fn new() -> Self {
        Blog {
            slug: String::from(""),
            date: String::from(""),
            file: String::from(""),
            file_path: String::from(""),
            title: String::from(""),
            tags: vec!(),
            excerpt: String::from(""),
            category: String::from(""),
            content: Some(String::from("")),
        }
    }
}

#[tokio::main]
async fn main () {
    fast_log::init(fast_log::Config::new()
        .console()
        .chan_len(Some(100000))
        .file_split(
            "target/logs/",
            LogSize::MB(1),
            RollingType::All,
            LogPacker{}
        )).unwrap();
    // fast_log::init("target/restore.log",
    //                    // 1000,
    //                    log::Level::Warn,
    //                    None,
    //                    true);
    let rb = init_rbatis().await;
    let rb: Arc<Rbatis> = Arc::new(rb);

    let categories = vec![
        Category { name: "backend", dir: "blog/backend/_posts" },
        Category { name: "frontend", dir: "blog/frontend/_posts" },
        Category { name: "rust", dir: "blog/rust/_posts" },
        Category { name: "server", dir: "blog/server/_posts" },
        Category { name: "diary", dir: "blog/diary/_posts" },
    ];

    let all_blogs = find_all_blogs(categories);

    let mut _i = 0;
    for blog in &all_blogs {
        let blog_res = save_blog(rb.clone(), blog).await;

        match blog_res {
            Ok(node) => {
                let body = blog.content.as_ref().unwrap().as_str();
                let res = save_node_content(
                    rb.clone(), node.nid,
                    body,
                    BodyFormat::Markdown,
                    blog.excerpt.as_str()
                ).await;

                if let Err(err) = res {
                    println!("Save node content failed: {}", err);
                }

                // println!("node: {:?}", node);
                let res = save_tags(rb.clone(), &blog.tags, node.nid).await;

                if let Err(err) = res {
                    println!("Save tags failed: {}", err);
                }

                let res = save_category(rb.clone(), &blog.category, node.nid).await;

                if let Err(err) = res {
                    println!("Save category failed: {}", err);
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

async fn save_blog(rb: Arc<Rbatis>, blog: &Blog) -> Result<Nodes, String> {
    let bundle = NodeBundle::Article;
    let mut rng = thread_rng();
    let hour = format!("{:02}", rng.gen_range(0..23));
    let minute = format!("{:02}", rng.gen_range(0..59));
    let second = format!("{:02}", rng.gen_range(0..59));
    let data_str = format!("{}T{}:{}:{}", &blog.date, hour, minute, second);
    let date = rbatis::DateTimeNative::from_str(&data_str).unwrap();
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
    let res = save_node(rb.clone(), &node, &bundle).await;

    match res {
        Ok(node) => Ok(node),
        Err(err) => Err(format!("Node save failed: {}", &blog.slug))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BlogMatter {
    title: Option<String>,
    tags: Option<String>,
    excerpt: Option<String>,
}

fn generate_blog(
    file_name: String,
    content: String,
    file_path: String,
    category: String
) -> Result<Blog, String> {
    if !is_valid_matter_content(&content) {
        warn!("[OICNP] Not valid matter content: {}", &file_name);
        return Err(format!("Not valid matter content: {}", &file_name));
    }

    let (date, slug) = generate_slug(&file_name);
    // println!("Matter parse: {}", &file_name);
    let matter = Matter::<YAML>::new();
    let res = matter.parse_with_struct::<BlogMatter>(&content)
        .expect("BlogMatter parse failed");
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
fn find_all_blogs(categories: Vec<Category>) -> Vec<Blog> {
    let blog_base = &G.config.blog_base;
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
