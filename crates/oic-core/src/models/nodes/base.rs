use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use loco_rs::prelude::*;
use crate::{
    models::RequestParamsUpdater,
    utils::utc_now,
    entities::prelude::*,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct NodeFilters {
    pub nid: Option<i64>,
    pub uuid: Option<String>,
    pub vid: Option<String>,
    pub bundle: Option<String>,
    pub title: Option<String>,
    pub viewed: Option<i32>,
    pub deleted: Option<String>,
    pub published_at: Option<DateTime>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default)]
#[serde(default)]
pub struct NodeReqParams {
    pub nid: Option<i64>,
    #[validate(required(message = "必须指定 vid"), length(min = 2, message = "vid 最少2个字符"))]
    pub vid: Option<String>,
    pub uuid: Option<String>,
    pub bundle: Option<String>,
    #[validate(required(message = "必须指定 title"), length(min = 2, message = "title 最少2个字符"))]
    pub title: Option<String>,
    #[validate(length(min = 2, message = "content 最少2个字符"))]
    pub content: Option<String>,
    pub viewed: Option<i32>,
    pub deleted: Option<String>,
    pub published_at: Option<DateTime>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
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
            item.published_at = Set(Some(*x));
        }
        if let Some(x) = &self.created_by {
            item.created_by = Set(*x);
        }
        if let Some(x) = &self.updated_by {
            item.updated_by = Set(*x);
        }
        if let Some(x) = &self.created_at {
            item.created_at = Set(*x);
        }
        if let Some(x) = &self.updated_at {
            item.updated_at = Set(Some(*x));
        } else {
            item.updated_at = Set(Some(utc_now()));
        }
        if let Some(x) = &self.deleted_at {
            item.deleted_at = Set(Some(*x));
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        item.nid = ActiveValue::NotSet;

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
