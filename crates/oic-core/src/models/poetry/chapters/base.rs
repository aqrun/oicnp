use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::utils::utc_now;
use crate::{RequestParamsUpdater, uuid, constants::DATE_TIME_FORMAT};

pub use crate::entities::poetry::{
  ChapterActiveModel,
  ChapterEntity,
  ChapterModel,
  ChapterColumn,
};


#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ChapterFilters {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub pid: Option<i32>,
    #[serde(rename(deserialize = "poetryId"))]
    pub poetry_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    #[serde(rename(deserialize = "wordCount"))]
    pub word_count: Option<i16>,
    pub weight: Option<i16>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

/// 创建 chapter 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct ChapterReqParams {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub pid: Option<i32>,
    #[serde(rename(deserialize = "poetryId"))]
    pub poetry_id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    #[serde(rename(deserialize = "wordCount"))]
    pub word_count: Option<i16>,
    pub weight: Option<i16>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

impl RequestParamsUpdater for ChapterReqParams {
    type ActiveModel = ChapterActiveModel;

    /// 根据非空正常数据更新
    fn update(&self, chapter: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            chapter.id = Set(*x);
        }

        if let Some(x) = &self.uuid {
            chapter.uuid = Set(String::from(x));
        }

        if let Some(x) = &self.pid {
            chapter.pid = Set(*x);
        }

        if let Some(x) = &self.poetry_id {
            chapter.poetry_id = Set(*x);
        }

        if let Some(x) = &self.title {
            chapter.title = Set(String::from(x));
        }
        if let Some(x) = &self.description {
            chapter.description = Set(String::from(x));
        }

        if let Some(x) = &self.content {
            chapter.content = Set(String::from(x));
        }

        if let Some(x) = &self.word_count {
            chapter.word_count = Set(*x);
        }

        if let Some(x) = &self.weight {
            chapter.weight = Set(*x);
        }

        if let Some(x) = &self.created_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                chapter.created_at = Set(x);
            }
        }

        if let Some(x) = &self.updated_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                chapter.updated_at = Set(Some(x));
            }
        } else {
            chapter.updated_at = Set(Some(utc_now()));
        }
    }

    ///
    /// 是创建操作需要再设置一些默认参数 如密码 uuid等
    /// 
    fn update_by_create(&self, chapter: &mut Self::ActiveModel) {
        if self.uuid.is_none() {
            chapter.uuid = Set(uuid!());
        }

        if self.created_at.is_none() {
            chapter.created_at = Set(utc_now());
        }
    }
}

pub type CreateChapterReqParams = ChapterReqParams;

///
/// 更新 chapter 参数
/// 
pub type UpdateChapterReqParams = ChapterReqParams;
///
/// 删除数据参数
/// 
pub type DeleteChapterReqParams = ChapterReqParams;

