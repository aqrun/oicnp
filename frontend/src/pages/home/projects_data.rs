use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

const PROJECTS_JSON: &'static str = include_str!("projects.json");
/// 解析本地JSON数据
pub static PROJECTS: Lazy<Vec<ProjectItem>> = Lazy::new(|| {
    let data: Vec<ProjectItem> = serde_json::from_str(PROJECTS_JSON)
        .unwrap_or(vec![]);
    data
});

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectItem {
    pub name: String,
    pub desc: String,
    pub href: String,
    pub tags: Vec<Tag>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub href: String,
}
