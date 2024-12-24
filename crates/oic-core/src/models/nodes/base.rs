use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug)]
pub struct NodeFilters {
    pub nid: Option<i64>,
    pub title: Option<String>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct CreateNodeReqParams {
    pub nid: Option<i64>,
    #[validate(required(message = "必须指定 vid"), length(min = 2, message = "vid 最少2个字符"))]
    pub vid: Option<String>,
    #[validate(required(message = "必须指定 title"), length(min = 2, message = "title 最少2个字符"))]
    pub title: Option<String>,
    #[validate(length(min = 2, message = "content 最少2个字符"))]
    pub content: Option<String>,
}

///
/// 更新 note 参数
/// 
pub type UpdateNodeReqParams = CreateNodeReqParams;
/// 删除数据参数
pub type DeleteNodeReqParams = CreateNodeReqParams;
