use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::utils::utc_now;
use crate::{RequestParamsUpdater, constants::DATE_TIME_FORMAT};

pub use crate::entities::poetry::{
  ChapterLineActiveModel,
  ChapterLineEntity,
  ChapterLineModel,
  ChapterLineColumn,
};


#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ChapterLineFilters {
    pub id: Option<i32>,
    #[serde(rename(deserialize = "chapterId"))]
    pub chapter_id: Option<i32>,
    #[serde(rename(deserialize = "lineNumber"))]
    pub line_number: Option<i32>,
    pub content: Option<String>,
    pub pinyin: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

/// 创建 chapter_line 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct ChapterLineReqParams {
    pub id: Option<i32>,
    #[serde(rename(deserialize = "chapterId"))]
    pub chapter_id: Option<i32>,
    #[serde(rename(deserialize = "lineNumber"))]
    pub line_number: Option<i32>,
    pub content: Option<String>,
    pub pinyin: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

impl RequestParamsUpdater for ChapterLineReqParams {
    type ActiveModel = ChapterLineActiveModel;

    /// 根据非空正常数据更新
    fn update(&self, chapter_line: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            chapter_line.id = Set(*x);
        }

        if let Some(x) = &self.chapter_id {
            chapter_line.chapter_id = Set(*x);
        }

        if let Some(x) = &self.line_number {
            chapter_line.line_number = Set(*x);
        }

        if let Some(x) = &self.content {
            chapter_line.content = Set(String::from(x));
        }

        if let Some(x) = &self.pinyin {
            chapter_line.pinyin = Set(String::from(x));
        }

        if let Some(x) = &self.description {
            chapter_line.description = Set(String::from(x));
        }

        if let Some(x) = &self.notes {
            chapter_line.notes = Set(String::from(x));
        }

        if let Some(x) = &self.created_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                chapter_line.created_at = Set(x);
            }
        }

        if let Some(x) = &self.updated_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                chapter_line.updated_at = Set(Some(x));
            }
        } else {
            chapter_line.updated_at = Set(Some(utc_now()));
        }
    }

    ///
    /// 是创建操作需要再设置一些默认参数 如密码 uuid等
    /// 
    fn update_by_create(&self, chapter_line: &mut Self::ActiveModel) {
        if self.created_at.is_none() {
            chapter_line.created_at = Set(utc_now());
        }
    }
}

pub type CreateChapterLineReqParams = ChapterLineReqParams;

///
/// 更新 chapter_line 参数
/// 
pub type UpdateChapterLineReqParams = ChapterLineReqParams;
///
/// 删除数据参数
/// 
pub type DeleteChapterLineReqParams = ChapterLineReqParams;

