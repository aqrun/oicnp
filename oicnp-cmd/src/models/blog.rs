use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Category<'a> {
    pub name: &'a str,
    pub vid: &'a str,
    pub dir: &'a str,
    pub weight: i32,
    pub parent: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct MatterTaxonomy {
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogMatter {
    pub title: Option<String>,
    pub description: Option<String>,
    pub taxonomies: Option<MatterTaxonomy>,
}

