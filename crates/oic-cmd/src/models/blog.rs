use serde::{Deserialize, Serialize};
use oic_core::{models::nodes::CreateNodeReqParams, prelude::*};

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

impl Blog {
    pub fn to_create_node_req_params(&self) -> CreateNodeReqParams {
        let mut content = String::from("");

        if let Some(x) = &self.content {
            content = String::from(x);
        }

        let date = format!("{} 02:00:00", self.date);

        CreateNodeReqParams {
            nid: None,
            vid: Some(String::from(self.slug.as_str())),
            uuid: Some(uuid!()),
            bundle: Some(String::from("blog")),
            title: Some(String::from(self.title.as_str())),
            body: Some(String::from(content.as_str())),
            summary: Some(String::from(self.excerpt.as_str())),
            summary_format: Some(String::from("markdown")),
            body_format: Some(String::from("markdown")),
            published_at: Some(String::from(date.as_str())),
            created_at: Some(String::from(date.as_str())),
            created_by_username: Some(String::from("aqrun")),
            tag_vids: Some(self.tags.clone()),
            category_vids: Some(vec![String::from(self.category.as_str())]),
            ..Default::default()
        }
    }
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

