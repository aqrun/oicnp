use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use loco_rs::prelude::*;
use crate::{
    models::RequestParamsUpdater,
    utils::utc_now,
    entities::prelude::*,
    constants::DATE_TIME_FORMAT,
    uuid,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct NodeFilters {
    pub nid: Option<i64>,
    pub nids: Option<String>,
    pub uuid: Option<String>,
    pub vid: Option<String>,
    pub bundle: Option<String>,
    pub title: Option<String>,
    pub viewed: Option<i32>,
    pub deleted: Option<String>,
    #[serde(rename(deserialize = "publishedAt", serialize = "publishedAt"))]
    pub published_at: Option<String>,
    #[serde(rename(deserialize = "createdBy", serialize = "createdBy"))]
    pub created_by: Option<i64>,
    #[serde(rename(deserialize = "updatedBy", serialize = "updatedBy"))]
    pub updated_by: Option<i64>,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<String>,
    #[serde(rename(deserialize = "deletedAt", serialize = "deletedAt"))]
    pub deleted_at: Option<String>,
    #[serde(rename(deserialize = "categoryVids", serialize = "categoryVids"))]
    pub category_vids: Option<Vec<String>>,
    #[serde(rename(deserialize = "categoryIds", serialize = "categoryIds"))]
    pub category_ids: Option<Vec<i64>>,
    #[serde(rename(deserialize = "tagVids", serialize = "tagVids"))]
    pub tag_vids: Option<Vec<String>>,
    #[serde(rename(deserialize = "tagIds", serialize = "tagIds"))]
    pub tag_ids: Option<Vec<i64>>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct NodeReqParams {
    pub nid: Option<i64>,
    #[validate(required(message = "必须指定 vid"), length(min = 2, message = "vid 最少2个字符"))]
    pub vid: Option<String>,
    pub uuid: Option<String>,
    pub bundle: Option<String>,
    #[validate(required(message = "必须指定 title"), length(min = 2, message = "title 最少2个字符"))]
    pub title: Option<String>,
    #[validate(length(min = 2, message = "body 最少2个字符"))]
    pub body: Option<String>,
    pub summary: Option<String>,
    #[serde(rename(deserialize = "summaryFormat", serialize = "summaryFormat"))]
    pub summary_format: Option<String>,
    #[serde(rename(deserialize = "bodyFormat", serialize = "bodyFormat"))]
    pub body_format: Option<String>,
    pub viewed: Option<i32>,
    pub deleted: Option<String>,
    #[serde(rename(deserialize = "publishedAt", serialize = "publishedAt"))]
    pub published_at: Option<String>,
    #[serde(rename(deserialize = "createdBy", serialize = "createdBy"))]
    pub created_by: Option<i64>,
    #[serde(rename(deserialize = "updatedBy", serialize = "updatedBy"))]
    pub updated_by: Option<i64>,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "createdByUsername", serialize = "createdByUsername"))]
    pub created_by_username: Option<String>,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<String>,
    #[serde(rename(deserialize = "deletedAt", serialize = "deletedAt"))]
    pub deleted_at: Option<String>,
    #[serde(rename(deserialize = "tagVids", serialize = "tagVids"))]
    pub tag_vids: Option<Vec<String>>,
    #[serde(rename(deserialize = "categoryVids", serialize = "categoryVids"))]
    pub category_vids: Option<Vec<String>>,
    #[serde(rename(deserialize = "tagIds", serialize = "tagIds"))]
    pub tag_ids: Option<Vec<i64>>,
    #[serde(rename(deserialize = "categoryIds", serialize = "categoryIds"))]
    pub category_ids: Option<Vec<i64>>,
}

impl RequestParamsUpdater for NodeReqParams {
    type ActiveModel = NodeActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.nid {
            item.nid = Set(*x);
        }
        if let Some(x) = &self.vid {
            item.vid = Set(String::from(x));
        }
        if let Some(x) = &self.uuid {
            item.uuid = Set(String::from(x));
        }
        if let Some(x) = &self.bundle {
            item.bundle = Set(String::from(x));
        }
        if let Some(x) = &self.title {
            item.title = Set(String::from(x));
        }
        if let Some(x) = &self.viewed {
            item.viewed = Set(*x);
        }
        if let Some(x) = &self.deleted {
            item.deleted = Set(String::from(x));
        }
        if let Some(x) = &self.published_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                item.published_at = Set(Some(x));
            }
        }
        if let Some(x) = &self.created_by {
            item.created_by = Set(*x);
        }

        if let Some(x) = &self.updated_by {
            item.updated_by = Set(*x);
        }
        if let Some(x) = &self.created_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                item.created_at = Set(x);
            }
        }
        if let Some(x) = &self.updated_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                item.updated_at = Set(Some(x));
            }
        }
        if let Some(x) = &self.deleted_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                item.deleted_at = Set(Some(x));
            }
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        item.nid = ActiveValue::NotSet;

        if self.uuid.is_none() {
            item.uuid = Set(uuid!());
        }

        if item.created_at.is_not_set() {
            item.created_at = Set(utc_now());
        }
    }
}

pub type CreateNodeReqParams = NodeReqParams;
///
/// 更新 note 参数
/// 
pub type UpdateNodeReqParams = NodeReqParams;
/// 删除数据参数
pub type DeleteNodeReqParams = NodeReqParams;

#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct NodeDetailModel {
    pub nid: i64,
    pub vid: String,
    pub uuid: String,
    pub bundle: String,
    pub title: String,
    pub viewed: i32,
    pub deleted: String,
    #[serde(rename(deserialize = "publishedAt", serialize = "publishedAt"))]
    pub published_at: Option<DateTime>,
    #[serde(rename(deserialize = "createdBy", serialize = "createdBy"))]
    pub created_by: i64,
    #[serde(rename(deserialize = "updatedBy", serialize = "updatedBy"))]
    pub updated_by: i64,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: DateTime,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<DateTime>,
    #[serde(rename(deserialize = "deletedAt", serialize = "deletedAt"))]
    pub deleted_at: Option<DateTime>,
    #[serde(rename(deserialize = "summary", serialize = "summary"))]
    pub summary: String,
    #[serde(rename(deserialize = "summaryFormat", serialize = "summaryFormat"))]
    pub summary_format: String,
    #[serde(rename(deserialize = "body", serialize = "body"))]
    pub body: Option<String>,
    #[serde(rename(deserialize = "bodyFormat", serialize = "bodyFormat"))]
    pub body_format: Option<String>,
    #[serde(rename(deserialize = "authorUid", serialize = "authorUid"))]
    pub author_uid: Option<i64>,
    #[serde(rename(deserialize = "authorUsername", serialize = "authorUsername"))]
    pub author_username: Option<String>,
    #[serde(rename(deserialize = "authorNickname", serialize = "authorNickname"))]
    pub author_nickname: Option<String>,
    #[serde(rename(deserialize = "updatedByUsername", serialize = "updatedByUsername"))]
    pub updated_by_username: Option<String>,
    #[serde(rename(deserialize = "updatedByNickname", serialize = "updatedByNickname"))]
    pub updated_by_nickname: Option<String>,
    // 分类和标签数组
    pub categories: Vec<crate::entities::category::Model>,
    pub tags: Vec<crate::entities::tag::Model>,
}
