use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct ToolListParams {
    pub page: Option<u64>,
    #[serde(rename(deserialize = "pageSize", serialize = "pageSize"))]
    pub page_size: Option<u64>,
    #[serde(rename(deserialize = "catVid", serialize = "catVid"))]
    pub cat_vid: Option<String>,
    #[serde(rename(deserialize = "tagVid", serialize = "tagVid"))]
    pub tag_vid: Option<String>,
    pub is_category_page: Option<bool>,
    pub is_tag_page: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct ToolItem {
  pub name: String,
  pub url: String,
  pub description: String,
  pub logo: String,
  pub language: String,
  pub category: String,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct ToolCategories {
  pub id: String,
  pub name: String,
}

#[derive(Deserialize)]
struct ToolsData {
    tools: Vec<ToolItem>,
}

// 使用 include_str! 在编译时嵌入 JSON，运行时解析
const TOOLS_JSON: &str = include_str!("tools.json");

// 全局缓存所有工具数据
pub static ALL_TOOLS: Lazy<Vec<ToolItem>> = Lazy::new(|| {
    let data: ToolsData = serde_json::from_str(TOOLS_JSON)
        .expect("Failed to parse tools.json");
    data.tools
});

// 按分类获取工具
pub fn get_tools_by_category(category: &str) -> Vec<ToolItem> {
    if category == "all" {
        ALL_TOOLS.clone()
    } else {
        ALL_TOOLS.iter()
            .filter(|item| item.category == category)
            .cloned()
            .collect()
    }
}
