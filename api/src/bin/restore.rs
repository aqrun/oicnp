/// 历史数据恢复
///
use std::path::PathBuf;
use std::fs;
use std::io::Read;
use api::utils::G;
use api::constants::BLOG_BASE;
use regex::Regex;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use api::dbs::{get_connection_pool, ConnectionPool};
use std::sync::Arc;
use diesel::pg::PgConnection;
use api::schema::taxonomy;
use diesel::prelude::*;
use api::models::{
    Taxonomy, NewTaxonomy,
};

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
    let blog_base = G.get(BLOG_BASE).unwrap();
    let pool = get_connection_pool();

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
                save_tags(pool.clone(), &blog.tags);
                // save_category(&conn, &blog.category); 
                // save_blog(&conn, &blog);
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

fn fetch_tag(pool: ConnectionPool, tag_name: &str) -> Result<Taxonomy, String> {
    use taxonomy::dsl;
    let conn = pool.get().unwrap();
    
    let tag_query = dsl::taxonomy
	.filter(dsl::name.eq(tag_name))
	.filter(dsl::bundle.eq("tag"))
	.first::<Taxonomy>(&conn);
}

fn save_tag(pool: ConnectionPool, tag_name: &str) -> Result<Taxonomy, String> {
    use taxonomy::dsl;
    let conn = pool.get().unwrap();
    let new_tag = Taxonomy {
	vid: String::from(tag_name),
	pid: 0,
	bundle: String::from("tag"),
	name: String::from(tag_name),
	description: String::from(""),
	description_format: String::from(""),
	weight: 0,
    };
    let query = diesel::insert_into(taxonomy::table)
	.values(&new_tag)
	.get_result(&conn);
    match query {
	Ok(tag) => Ok(tag),
	Err(_) => Err("Save tag data failed"),
    }
}

fn save_tags(pool: ConnectionPool, tags: &Vec<String>) -> Result<Vec<Taxonomy>, String> {
    use taxonomy::dsl;
    let conn = pool.get().unwrap();
    let tags_list: Vec<Taxonomy> = vec!();
    for tag_name in tags {
	match fetch_tag(pool.clone(), &tag_name) {
	    Ok(tag) => tags_list.push(tag),
	    Err(_) => return save_tag(pool.clone(), &tag_name)
	}
    }
    Ok(tags_list)
}

fn save_category(conn: &PgConnection, category: &Category) {

}

fn save_blog(conn: &PgConnection, blog: &Blog) {
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
