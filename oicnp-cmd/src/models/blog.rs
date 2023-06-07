use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Category<'a> {
    pub name: &'a str,
    pub dir: &'a str,
    pub weight: i32,
    pub parent: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    pub slug: String,
    pub date: String,
    pub file: String,
    pub file_path: String,
    pub title: String,
    pub tags: Vec<String>,
    pub excerpt: String,
    pub category: String,
    pub content: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogMatter {
    pub title: Option<String>,
    pub tags: Option<String>,
    pub excerpt: Option<String>,
}
