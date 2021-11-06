/// 历史数据恢复
///
use std::env;

struct Category {
    name: String,
    dir: String,
}

struct Blog {
    slug: String,
    date: String,
    file: String,
    file_path: String,
    title: String,
    tags: Vec<String>,
    excerpt: String,
    category: Category,
    content: Option<String>,
}

fn main () {
    let categories = [
        Category { name: "backend".to_String(), dir: "blog/backend/_posts".to_String() },
        Category { name: "frontend".to_String(), dir: "blog/frontend/_posts".to_String() },
        Category { name: "rust".to_String(), dir: "blog/rust/_posts".to_String() },
        Category { name: "server".to_String(), dir: "blog/server/_posts".to_String() },
        Category { name: "diary".to_String(), dir: "blog/diary/_posts".to_String() },
    ];
    let cwd = env::current_dir().unwrap();
    let allBlogs: Vec<Blog> = vec!();
    println!("{}", cwd.display());
}