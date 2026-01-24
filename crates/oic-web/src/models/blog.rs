use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct BlogListParams {
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
