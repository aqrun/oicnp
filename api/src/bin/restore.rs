/// 历史数据恢复
///
use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::Read;

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

const BLOG_BASE: &'static str = "/Users/aqrun/workspace/github.com/aqrun/aqrun.github.io";

fn main () {
    let categories = [
        Category { name: "backend", dir: "blog/backend/_posts" },
        Category { name: "frontend", dir: "blog/frontend/_posts" },
        Category { name: "rust", dir: "blog/rust/_posts" },
        Category { name: "server", dir: "blog/server/_posts" },
        Category { name: "diary", dir: "blog/diary/_posts" },
    ];
    // let cwd = env::current_dir().unwrap();
    let base = PathBuf::from(BLOG_BASE);
    let mut allBlogs: Vec<Blog> = vec!();
    let mut index = 0;

    for item in categories {
        let mut dir = base.clone();
        dir.push(item.dir);

        let entries = fs::read_dir(dir).expect("Read dir failed");

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
                allBlogs.push(blog);
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
    println!("{:?}", allBlogs);
}

fn generate_blog<'a>(file_name: String, content: String, file_path: String) -> Result<Blog<'a>, String> {
    let category = Category {
        name: "backend",
        dir: "blog"
    };

    let blog = Blog {
        slug: file_name.clone(),
        date: file_name.clone(),
        file: String::from(""),
        file_path,
        title: String::from(""),
        tags: vec!(),
        excerpt: String::from(""),
        category,
        content: Some(content.clone()),
    };
    Ok(blog)
}