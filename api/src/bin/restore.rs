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
use api::schema::taxonomy;
use api::models::{
    Taxonomy, Node,
};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

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

fn main () {
    let blog_base = &G.config.blog_base;
    let rb = init_rbatis().await;
    let rb: Arc<Rbatis> = Arc::new(rb);

    let categories = [
        Category { name: "backend", dir: "blog/backend/_posts" },
        Category { name: "frontend", dir: "blog/frontend/_posts" },
        Category { name: "rust", dir: "blog/rust/_posts" },
        Category { name: "server", dir: "blog/server/_posts" },
        Category { name: "diary", dir: "blog/diary/_posts" },
    ];
    // let cwd = env::current_dir().unwrap();
    let base = PathBuf::from(blog_base);
    let mut allBlogs: Vec<Blog> = vec!();
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
                // allBlogs.push(blog);

                save_blog(rb.clone(), &blog);
                // save_tags(rb.clone(), &blog.tags);
                // save_category(rb.clone(), &blog.category);
            }

            index += 1;

            if index > 1 {
                break;
            }
            // println!("{}", content)
        }

        if index > 1 {
            break;
        }
    }
}

fn fetch_tag(rb: Arc<Rbatis>, tag_name: &str) -> Result<Taxonomy, String> {
    use taxonomy::dsl;
    let conn = pool.get().unwrap();
    
    let tag_query = dsl::taxonomy
	.filter(dsl::name.eq(tag_name))
	.filter(dsl::bundle.eq("tag"))
	.first::<Taxonomy>(&conn);
}

fn save_tag(rb: Arc<Rbatis>, tag_name: &str) -> Result<Taxonomy, String> {
    use taxonomy::dsl;
    let conn = pool.get().unwrap();
    let new_tag = Taxonomy {
        tid: None,
        vid: Some(String::from(tag_name)),
        pid: Some(0),
        bundle: Some(String::from("tag")),
        name: Some(String::from(tag_name)),
        description: Some(String::from("")),
        description_format: Some(String::from("")),
        weight: Some(0),
    };
    let query = diesel::insert_into(taxonomy::table)
        .values(&new_tag)
        .get_result(&conn);
    match query {
        Ok(tag) => Ok(tag),
        Err(_) => Err(String::from("Save tag data failed")),
    }
}

fn save_tags(rb: Arc<Rbatis>, tags: &Vec<String>) -> Result<Vec<Taxonomy>, String> {
    use taxonomy::dsl;
    let mut tags_list: Vec<Taxonomy> = vec!();

    for tag_name in tags {
        match fetch_tag(rb.clone(), &tag_name) {
            Ok(tag) => tags_list.push(tag),
            Err(_) => {
                let temp_tag = save_tag(rb.clone(), &tag_name)?;
                tags_list.push(temp_tag);
            }
        }
    }
    Ok(tags_list)
}

fn save_category(rb: Arc<Rbatis>, category: &Category) {

}

fn save_blog(rb: Arc<Rbatis>, blog: &Blog) {
    let node = Node {
        nid: None,
        vid: Some(String::from(&blog.slug)),
        uid: None,
        bundle: Some(String::from("blog")),
        title: Some(String::from(&blog.title)),
        deleted: Some(false),
        created_at: None,
        created_by: None,
        updated_at: None,
        updated_by: None
    };
    rb.save(&node, &[]).await;
}


fn generate_blog<'a>(file_name: String, content: String, file_path: String) -> Result<Blog<'a>, String> {
    let category = Category {
        name: "backend",
        dir: "blog"
    };

    let (date, slug) = generate_slug(&file_name);

    let matter = Matter::<YAML>::new();
    let res = matter.parse(&content);
    let data = res.data.as_ref().unwrap();
    // let layout = &data["layout"].as_string().unwrap();
    let title = &data["title"].as_string().unwrap();
    let tags = &data["tags"].as_string().unwrap();
    let tag_arr = tags.split(" ")
        .map(|item| String::from(item))
        .collect();
    let excerpt = &data["excerpt"].as_string().unwrap();
    let con = res.content;

    let blog = Blog {
        slug,
        date,
        file: String::from(&file_name),
        file_path,
        title: String::from(title),
        tags: tag_arr,
        excerpt: String::from(excerpt),
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
