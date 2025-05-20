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
pub struct TagFilters {
    #[serde(rename(deserialize = "tagId", serialize = "tagId"))]
    pub tag_id: Option<i64>,
    #[serde(rename(deserialize = "tagVid", serialize = "tagVid"))]
    pub tag_vid: Option<String>,
    #[serde(rename(deserialize = "tagName", serialize = "tagName"))]
    pub tag_name: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(deserialize = "tagCount", serialize = "tagCount"))]
    pub tag_count: Option<i64>,
}

/// 创建 tag 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct TagReqParams {
    #[serde(rename(deserialize = "tagId", serialize = "tagId"))]
    pub tag_id: Option<i64>,
    #[serde(rename(deserialize = "tagVid", serialize = "tagVid"))]
    pub tag_vid: Option<String>,
    #[validate(required(message = "必须指定 tagName"), length(min = 2, message = "tagName 最少2个字符"))]
    #[serde(rename(deserialize = "tagName", serialize = "tagName"))]
    pub tag_name: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(deserialize = "tagCount", serialize = "tagCount"))]
    pub tag_count: Option<i64>,
}

impl RequestParamsUpdater for TagReqParams {
    type ActiveModel = TagActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.tag_id {
            item.tag_id = Set(*x);
        }
        if let Some(x) = &self.tag_vid {
            item.tag_vid = Set(String::from(x));
        }
        if let Some(x) = &self.tag_name {
            item.tag_name = Set(String::from(x));
        }
        if let Some(x) = &self.weight {
            item.weight = Set(*x);
        }
        if let Some(x) = &self.tag_count {
            item.tag_count = Set(*x);
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        item.tag_id = ActiveValue::NotSet;
    }
}

pub type CreateTagReqParams = TagReqParams;
pub type UpdateTagReqParams = TagReqParams;
pub type DeleteTagReqParams = TagReqParams;

