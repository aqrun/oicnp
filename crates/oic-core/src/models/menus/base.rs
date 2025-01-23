use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug)]
pub struct MenuFilters {
    pub id: Option<i64>,
    pub title: Option<String>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct CreateMenuReqParams {
    #[validate(required(message = "必须指定 id"))]
    pub id: Option<i64>,
    pub mid: Option<String>,
    pub pid: Option<String>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub r#type: Option<String>,
    pub query: Option<String>,
    pub weight: Option<i32>,
    pub api: Option<String>,
    pub status: Option<String>,
    pub method: Option<String>,
    pub component: Option<String>,
    pub visible: Option<String>,
    pub is_cache: Option<String>,
    pub log_method: Option<String>,
    pub data_cache_method: Option<String>,
    pub is_frame: Option<String>,
    pub data_scope: Option<String>,
    pub i18n: Option<String>,
    pub remark: Option<String>,
}

///
/// 更新 note 参数
/// 
pub type UpdateMenuReqParams = CreateMenuReqParams;
/// 删除数据参数
pub type DeleteMenuReqParams = CreateMenuReqParams;
