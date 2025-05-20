use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use crate::{
    models::RequestParamsUpdater,
    entities::prelude::*,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct CategoryFilters {
    #[serde(rename(deserialize = "catId", serialize = "catId"))]
    pub cat_id: Option<i64>,
    #[serde(rename(deserialize = "catVid", serialize = "catVid"))]
    pub cat_vid: Option<String>,
    #[serde(rename(deserialize = "catPid", serialize = "catPid"))]
    pub cat_pid: Option<i64>,
    #[serde(rename(deserialize = "catName", serialize = "catName"))]
    pub cat_name: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(deserialize = "catDesc", serialize = "catDesc"))]
    pub cat_desc: Option<String>,
    #[serde(rename(deserialize = "catDescFormat", serialize = "catDescFormat"))]
    pub cat_desc_format: Option<String>,
}

/// 创建 category 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct CategoryReqParams {
    #[serde(rename(deserialize = "catId", serialize = "catId"))]
    pub cat_id: Option<i64>,
    #[serde(rename(deserialize = "catVid", serialize = "catVid"))]
    pub cat_vid: Option<String>,
    #[validate(required(message = "必须指定 catName"), length(min = 2, message = "catName 最少2个字符"))]
    #[serde(rename(deserialize = "catName", serialize = "catName"))]
    pub cat_name: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(deserialize = "catDesc", serialize = "catDesc"))]
    pub cat_desc: Option<String>,
    #[serde(rename(deserialize = "catDescFormat", serialize = "catDescFormat"))]
    pub cat_desc_format: Option<String>,
}

impl RequestParamsUpdater for CategoryReqParams {
    type ActiveModel = CategoryActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.cat_id {
            item.cat_id = Set(*x);
        }
        if let Some(x) = &self.cat_vid {
            item.cat_vid = Set(String::from(x));
        }
        if let Some(x) = &self.cat_name {
            item.cat_name = Set(String::from(x));
        }
        if let Some(x) = &self.weight {
            item.weight = Set(*x);
        }
        if let Some(x) = &self.cat_desc {
            item.cat_desc = Set(String::from(x));
        }
        if let Some(x) = &self.cat_desc_format {
            item.cat_desc_format = Set(String::from(x));
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        item.cat_id = ActiveValue::NotSet;
    }
}

pub type CreateCategoryReqParams = CategoryReqParams;
pub type UpdateCategoryReqParams = CategoryReqParams;
pub type DeleteCategoryReqParams = CategoryReqParams;

