use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct PoetryListParams {
    pub page: Option<u64>,
    #[serde(rename(deserialize = "pageSize", serialize = "pageSize"))]
    pub page_size: Option<u64>,
    #[serde(rename(deserialize = "catVid", serialize = "catVid"))]
    pub cat_vid: Option<String>,
    pub uuid: Option<String>,
}
