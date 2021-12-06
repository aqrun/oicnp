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
    Taxonomy, Node, NewNode, NewTaxonomy
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
struct Blog<'a> {
    slug: String,
    date: String,
    file: String,
    file_path: String,
    title: String,
    tags: Vec<String>,
    excerpt: String,
    category: Category<'a>,
    content: Option<String>,
}

impl<'a> Blog<'a> {
    fn new() -> Self {
        let category = Category {
            name: "backend",
            dir: "blog"
        };

        Blog {
            slug: String::from(""),
            date: String::from(""),
            file: String::from(""),
            file_path: String::from(""),
            title: String::from(""),
            tags: vec!(),
            excerpt: String::from(""),
            category,
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
        if _i < 1 {
            save_blog(rb.clone(), blog).await;
            // save_tags(rb.clone(), &blog.tags);
            // save_category(rb.clone(), &blog.category);
        }
        _i += 1;
    }
}

async fn fetch_tag(rb: Arc<Rbatis>, tag_name: &str) -> Result<Taxonomy, String> {
    let wrapper = rb.new_wrapper()
        .eq("name", tag_name)
        .eq("bundle", "tag");
    
    let result: Result<Option<Taxonomy>, Error> = rb.fetch_by_wrapper(wrapper)
        .await;

    if let Ok(res) = result {
        if let Some(tag) = res {
            return Ok(tag);
        }
    }
    Err(format!("Tag not exist, {}", tag_name))
}

async fn save_tag(rb: Arc<Rbatis>, tag_name: &str) -> Result<Taxonomy, String> {
    let new_tag = NewTaxonomy {
        vid: String::from(tag_name),
        pid: 0,
        bundle: String::from("tag"),
        name: String::from(tag_name),
        description: String::from(""),
        description_format: String::from(""),
        weight: 0,
    };
    let res = rb.save(&new_tag, &[]).await;

    if let Ok(_) = res {
        return fetch_tag(rb.clone(), tag_name).await;
    }
    Err(format!("Save tag failed: {}", tag_name))
}

async fn save_tags(rb: Arc<Rbatis>, tags: &Vec<String>) -> Result<Vec<Taxonomy>, String> {
    let mut tags_list: Vec<Taxonomy> = vec!();

    for tag_name in tags {
        let res = fetch_tag(rb.clone(), &tag_name).await;
        match res {
            Ok(tag) => tags_list.push(tag),
            Err(_) => {
                let temp_tag = save_tag(rb.clone(), &tag_name).await?;
                tags_list.push(temp_tag);
            }
        }
    }
    Ok(tags_list)
}

// async fn save_category<'a>(rb: Arc<Rbatis>, category: &'a Category) {
//
// }

async fn save_blog<'a>(rb: Arc<Rbatis>, blog: &'a Blog<'a>) {
    let node = NewNode {
        vid: String::from(&blog.slug),
        uid: 1,
        bundle: String::from("blog"),
        title: String::from(&blog.title),
        deleted: false,
        created_by: 1,
        updated_by: 1
    };
    rb.save(&node, &[]).await;
}

#[derive(Serialize, Deserialize, Debug)]
struct BlogMatter {
    title: Option<String>,
    tags: Option<String>,
    excerpt: Option<String>,
}

fn generate_blog<'a>(file_name: String, content: String, file_path: String) -> Result<Blog<'a>, String> {
    let category = Category {
        name: "backend",
        dir: "blog"
    };

    let (date, slug) = generate_slug(&file_name);
    println!("Matter parse: {}", &file_name);
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
fn find_all_blogs<'a>(categories: Vec<Category>) -> Vec<Blog<'a>> {
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
                String::from(entry.path().to_str().expect("Invalid str"))
            ) {
                all_blogs.push(blog);
            }

            index += 1;
            // println!("{}", content)
        }
    }

    all_blogs
}
