use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::utils::utc_now;
use crate::{RequestParamsUpdater, uuid, constants::DATE_TIME_FORMAT};

pub use crate::entities::poetry::{
  PoetryActiveModel,
  PoetryEntity,
  PoetryModel,
  PoetryColumn,
};


#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct PoetryFilters {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub title: Option<String>,
    #[serde(rename(deserialize = "authorId"))]
    pub author_id: Option<i32>,
    pub dynasty: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(deserialize = "hotWeight"))]
    pub hot_weight: Option<i32>,
    pub content: Option<String>,
    #[serde(rename(deserialize = "wordCount"))]
    pub word_count: Option<i32>,
    pub tags: Option<String>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

/// 创建 poetry 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct PoetryReqParams {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub title: Option<String>,
    #[serde(rename(deserialize = "authorId"))]
    pub author_id: Option<i32>,
    pub dynasty: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(deserialize = "hotWeight"))]
    pub hot_weight: Option<i32>,
    pub content: Option<String>,
    #[serde(rename(deserialize = "wordCount"))]
    pub word_count: Option<i32>,
    pub tags: Option<String>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

impl RequestParamsUpdater for PoetryReqParams {
    type ActiveModel = PoetryActiveModel;

    /// 根据非空正常数据更新
    fn update(&self, poetry: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            poetry.id = Set(*x);
        }

        if let Some(x) = &self.uuid {
            poetry.uuid = Set(String::from(x));
        }

        if let Some(x) = &self.title {
            poetry.title = Set(String::from(x));
        }

        if let Some(x) = &self.author_id {
            poetry.author_id = Set(*x);
        }

        if let Some(x) = &self.dynasty {
            poetry.dynasty = Set(String::from(x));
        }

        if let Some(x) = &self.weight {
            poetry.weight = Set(*x);
        }

        if let Some(x) = &self.hot_weight {
            poetry.hot_weight = Set(*x as i16);
        }

        if let Some(x) = &self.content {
            poetry.content = Set(String::from(x));
        }

        if let Some(x) = &self.word_count {
            poetry.word_count = Set(*x as i16);
        }

        if let Some(x) = &self.tags {
            poetry.tags = Set(String::from(x));
        }

        if let Some(x) = &self.created_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                poetry.created_at = Set(x);
            }
        }

        if let Some(x) = &self.updated_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                poetry.updated_at = Set(Some(x));
            }
        } else {
            poetry.updated_at = Set(Some(utc_now()));
        }
    }

    ///
    /// 是创建操作需要再设置一些默认参数 如密码 uuid等
    /// 
    fn update_by_create(&self, poetry: &mut Self::ActiveModel) {
        if self.uuid.is_none() {
            poetry.uuid = Set(uuid!());
        }

        if self.created_at.is_none() {
            poetry.created_at = Set(utc_now());
        }
    }
}

pub type CreatePoetryReqParams = PoetryReqParams;

///
/// 更新 poetry 参数
/// 
pub type UpdatePoetryReqParams = PoetryReqParams;
///
/// 删除数据参数
/// 
pub type DeletePoetryReqParams = PoetryReqParams;
