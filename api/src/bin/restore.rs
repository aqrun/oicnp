/// 历史数据恢复
///
use std::path::PathBuf;
use std::fs;
use std::io::Read;
use api::services::G;
use regex::Regex;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use api::dbs::{init_rbatis};
use std::sync::Arc;
use api::models::{
    Taxonomy, Node, NewNode, NewTaxonomy,
    NodeTagsMap, NodeCategoryMap, NodeBody,
};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use rbatis::Error;
use serde::{Deserialize, Serialize};

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
        if let Ok(node) = save_blog(rb.clone(), blog).await {
            let res = save_blog_content(rb.clone(), node.nid, blog).await;

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
        }
    }

    println!("Restore completed with {} data", _i);
}

async fn save_blog_content(rb: Arc<Rbatis>, nid: i32, blog: &Blog) -> Result<String, String> {
    rb.remove_by_column::<NodeBody, _>("nid", nid).await;
    let content = blog.content.as_ref().unwrap();
    let node_body = NodeBody {
        nid,
        summary: String::from(&blog.excerpt),
        body: content.to_string(),
        body_format: String::from("markdown"),
    };
    let res = rb.save(&node_body, &[]).await;
    println!("{}", &blog.excerpt);
    match res {
        Ok(_) => Ok(format!("Body save success")),
        Err(err) => Err(format!("Body save failed, {}", err.to_string())),
    }
}

async fn find_taxonomy(rb: Arc<Rbatis>, name: &str, bundle: &str) -> Result<Taxonomy, String> {
    let wrapper = rb.new_wrapper()
        .eq("name", name)
        .eq("bundle", bundle);
    
    let result: Result<Option<Taxonomy>, Error> = rb.fetch_by_wrapper(wrapper)
        .await;

    if let Ok(res) = result {
        if let Some(taxonomy) = res {
            return Ok(taxonomy);
        }
    }
    Err(format!("Taxonomy not exist, {}", name))
}

async fn save_tag(rb: Arc<Rbatis>, tag_name: &str) -> Result<Taxonomy, String> {
    if let Ok(tag) = find_taxonomy(rb.clone(), &tag_name, "tag").await {
        return Ok(tag);
    }

    let new_tag = NewTaxonomy {
        vid: String::from(tag_name),
        pid: 0,
        bundle: String::from("tag"),
        name: String::from(tag_name),
        description: String::from(""),
        description_format: String::from(""),
        weight: 0,
    };

    if let Ok(_res) = rb.save(&new_tag, &[]).await {
        if let Ok(tag) = find_taxonomy(rb.clone(), tag_name, "tag").await {
            return Ok(tag);
        }
    }

    Err(format!("Save tag failed: {}", tag_name))
}

async fn find_tag_map(rb: Arc<Rbatis>, nid: i32, tid: i32) -> Result<NodeTagsMap, String> {
    let w = rb.new_wrapper()
        .eq("bundle", "blog")
        .eq("nid", nid)
        .eq("tid", tid);
    let res: Result<Option<NodeTagsMap>, Error> = rb.fetch_by_wrapper(w.clone()).await;

    if let Ok(res) = res {
        if let Some(res) = res {
            return Ok(res);
        }
    }
    Err(format!("map not exist"))
}

async fn save_node_tag_map(rb: Arc<Rbatis>, nid: i32, tid: i32) -> Result<NodeTagsMap, String> {
    if let Ok(map) = find_tag_map(rb.clone(), nid, tid).await {
        return Ok(map);
    }

    let map = NodeTagsMap {
        bundle: "blog".to_string(),
        nid,
        tid
    };
    if let Err(err) = rb.save(&map, &[]).await {
        return Err(err.to_string());
    }

    if let Ok(map) = find_tag_map(rb.clone(), nid, tid).await {
        return Ok(map);
    }
    Err(format!("Tag map save failed"))
}

async fn save_tags(rb: Arc<Rbatis>, tags: &Vec<String>, nid: i32) -> Result<Vec<Taxonomy>, String> {
    let mut tags_list: Vec<Taxonomy> = vec!();

    for tag_name in tags {
        if let Ok(tag) = save_tag(rb.clone(), &tag_name).await {
            let res = save_node_tag_map(rb.clone(), nid, tag.tid).await;

            match res {
                Ok(_map) => tags_list.push(tag),
                Err(err) => return Err(err),
            }

        }
    }
    Ok(tags_list)
}

async fn save_category_data(rb: Arc<Rbatis>, category_name: &str) -> Result<Taxonomy, String> {
    if let Ok(cat) = find_taxonomy(rb.clone(), &category_name, "category").await {
        return Ok(cat);
    }

    let new_cat = NewTaxonomy {
        vid: String::from(category_name),
        pid: 0,
        bundle: String::from("category"),
        name: String::from(category_name),
        description: String::from(""),
        description_format: String::from(""),
        weight: 0,
    };

    if let Ok(_res) = rb.save(&new_cat, &[]).await {
        if let Ok(cat) = find_taxonomy(rb.clone(), category_name, "category").await {
            return Ok(cat);
        }
    }

    Err(format!("Save Category failed: {}", category_name))
}

async fn find_node_category_map(rb: Arc<Rbatis>, nid: i32, tid: i32) -> Result<NodeCategoryMap, String> {
    let w = rb.new_wrapper()
        .eq("bundle", "blog")
        .eq("nid", nid)
        .eq("tid", tid);
    let res: Result<Option<NodeCategoryMap>, Error> = rb.fetch_by_wrapper(w.clone()).await;

    if let Ok(res) = res {
        if let Some(res) = res {
            return Ok(res);
        }
    }
    Err(format!("map not exist"))
}

async fn save_node_category_map(rb: Arc<Rbatis>, nid: i32, tid: i32) -> Result<NodeCategoryMap, String> {
    if let Ok(map) = find_node_category_map(rb.clone(), nid, tid).await {
        return Ok(map);
    }

    let map = NodeCategoryMap {
        bundle: "blog".to_string(),
        nid,
        tid
    };
    if let Err(err) = rb.save(&map, &[]).await {
        return Err(err.to_string());
    }

    if let Ok(map) = find_node_category_map(rb.clone(), nid, tid).await {
        return Ok(map);
    }
    Err(format!("Tag map save failed"))
}

async fn save_category(rb: Arc<Rbatis>, category_name: &str, nid: i32) -> Result<Taxonomy, String> {
    if let Ok(cat) = save_category_data(rb.clone(), category_name).await {
        let _res = save_node_category_map(rb.clone(), nid, cat.tid).await?;

        return Ok(cat);
    }

    Err(format!("Save Category failed: {}", category_name))
}

async fn find_blog_by_vid(rb: Arc<Rbatis>, vid: &str) -> Result<Node, String> {
    let w = rb.new_wrapper()
        .eq("vid", vid)
        .eq("bundle", "blog");
    let node: Result<Option<Node>, Error> = rb.fetch_by_wrapper(w).await;

    if let Ok(node) = node {
        if let Some(node) = node {
            return Ok(node);
        }
    }
    Err(format!("Node not exist: {}", vid))
}

async fn save_blog(rb: Arc<Rbatis>, blog: &Blog) -> Result<Node, String> {
    if let Ok(node) = find_blog_by_vid(rb.clone(), &blog.slug).await {
        println!("Blog already exist: {}", &blog.slug);
        return Ok(node);
    }

    let node = NewNode {
        vid: String::from(&blog.slug),
        uid: 1,
        bundle: String::from("blog"),
        title: String::from(&blog.title),
        deleted: false,
        created_by: 1,
        updated_by: 1
    };
    let res = rb.save(&node, &[]).await;

    if let Err(err) = res {
        return Err(err.to_string());
    }

    if let Ok(node) = find_blog_by_vid(rb.clone(), &blog.slug).await {
        println!("Saved blog: {}", &blog.slug);
        return Ok(node);
    }

    Err(format!("Node save failed: {}", &blog.slug))
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
        println!("Not valid matter content: {}", &file_name);
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


fn generate_slug(file_name: &str) -> (String, String) {
    let mut file_arr: Vec<&str> = file_name.split(".").collect();
    file_arr.pop();
    let new_file_name = file_arr.join("-");
    file_arr = new_file_name.split("-").collect();
    let year = file_arr[0];
    let month = file_arr[1];
    let day = file_arr[2]; // .parse::<i32>().expect("Day error");
    let date = format!("{}-{}-{}", year, month, day);

    let re = Regex::new(r"[\.+\s]+").unwrap();
    let source_title = file_arr[3..].join("-");
    let title = re.replace_all(&source_title, "-");
    let slug = format!("{}-{}", date, title);

    let res = (date, slug);
    return res;
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

fn is_valid_matter_content(content: &str) -> bool {
    let reg_matter = Regex::new(r#"---([\s\S]*)---"#)
        .expect("Matter reg not valid");
    reg_matter.is_match(content)
}