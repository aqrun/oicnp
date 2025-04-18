use crate::constants::{CATEGORIES, VID_READING};
use crate::models::{Blog, BlogMatter, Category, MatterTaxonomy};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use oic_core::utils::{generate_slug, is_valid_matter_content};
use oic_core::prelude::{
    anyhow::{anyhow, Result},
    chrono::prelude::*,
    fast_log::{
        self,
        consts::LogSize,
        plugin::{file_split::RollingType, packer::LogPacker},
    },
    log::{error, info, warn},
    serde_json,
};
use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub async fn run(_format: &str, blog_base: &str, dist_file: &str) {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .chan_len(Some(100000))
            .file_split(
                "target/logs/",
                LogSize::MB(1),
                RollingType::All,
                LogPacker {},
            ),
    )
    .unwrap();

    info!(
        "\n\n\n[OICP] Blog handle start at {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    // let db = DB.get_or_init(establish_connection).await;
    let categories = &CATEGORIES;

    let all_blogs = find_all_blogs(categories, blog_base);
    // println!("all_blogs {}", serde_json::to_string(&all_blogs[0..2]).unwrap());
    match save_to_file(&all_blogs, dist_file) {
        Err(err) => {
            error!("[OICNP] Save to file failed, {:?}", err);
        }
        _ => {
            info!(
                "\n[OICNP] Blog save completed! total: {}",
                &all_blogs.capacity()
            );
        }
    }
    info!(
        "\n[OICNP] Blog handle end at: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
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
        warn!(
            "[OICNP] Not valid matter content: {}{}",
            &file_path, &file_name
        );
        return Err(anyhow!("[OICNP]Not valid matter content: {}", &file_name));
    }

    let (date, slug) = generate_slug(&file_name);
    // println!("Matter parse: {}", &file_name);
    let matter = Matter::<YAML>::new();
    let res = match matter.parse_with_struct::<BlogMatter>(&content) {
        Some(data) => data,
        _ => {
            println!("[OICNP]Matter parse failed: {}{}", &file_path, &file_name);
            return Err(anyhow!(
                "[OICNP]Matter parse failed: {}{}",
                &file_path,
                &file_name
            ));
        }
    };
    // println!("{:?}", res.data);
    let data = res.data;
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
fn find_all_blogs(categories: &Vec<Category>, blog_base: &str) -> Vec<Blog> {
    let base = PathBuf::from(blog_base);
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

    all_blogs
}

