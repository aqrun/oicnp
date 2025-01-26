use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use crate::utils::{default_string, default_i32};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug)]
pub struct MenuFilters {
    pub id: Option<i32>,
    pub title: Option<String>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct CreateMenuReqParams {
    #[serde(default = "default_string")]
    pub mid: String,
    #[serde(default = "default_string")]
    pub pid: String,
    #[serde(default = "default_string")]
    pub path: String,
    #[serde(default = "default_string")]
    pub name: String,
    #[serde(default = "default_string")]
    pub icon: String,
    #[serde(default = "default_i32")]
    pub weight: i32,
    #[serde(default = "default_string")]
    pub api: String,
    #[serde(default = "default_string")]
    pub status: String,
    #[serde(default = "default_string")]
    pub visible: String,
    #[serde(default = "default_string")]
    pub is_cache: String,
    #[serde(default = "default_string")]
    pub is_frame: String,
    #[serde(default = "default_string")]
    pub remark: String,
}

///
/// 更新 note 参数
/// 
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct UpdateMenuReqParams {
    pub id: i32,
    #[serde(default = "default_string")]
    pub mid: String,
    #[serde(default = "default_string")]
    pub pid: String,
    #[serde(default = "default_string")]
    pub path: String,
    #[serde(default = "default_string")]
    pub name: String,
    #[serde(default = "default_string")]
    pub icon: String,
    #[serde(default = "default_i32")]
    pub weight: i32,
    #[serde(default = "default_string")]
    pub api: String,
    #[serde(default = "default_string")]
    pub status: String,
    #[serde(default = "default_string")]
    pub visible: String,
    #[serde(default = "default_string")]
    pub is_cache: String,
    #[serde(default = "default_string")]
    pub is_frame: String,
    #[serde(default = "default_string")]
    pub remark: String,
}
/// 删除数据参数
pub type DeleteMenuReqParams = UpdateMenuReqParams;
