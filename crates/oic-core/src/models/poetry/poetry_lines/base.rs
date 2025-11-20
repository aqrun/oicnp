use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::utils::utc_now;
use crate::{RequestParamsUpdater, constants::DATE_TIME_FORMAT};

pub use crate::entities::poetry::{
  PoetryLineActiveModel,
  PoetryLineEntity,
  PoetryLineModel,
  PoetryLineColumn,
};


#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct PoetryLineFilters {
    pub id: Option<i32>,
    #[serde(rename(deserialize = "poetryId"))]
    pub poetry_id: Option<i32>,
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

/// 创建 poetry_line 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct PoetryLineReqParams {
    pub id: Option<i32>,
    #[serde(rename(deserialize = "poetryId"))]
    pub poetry_id: Option<i32>,
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

impl RequestParamsUpdater for PoetryLineReqParams {
    type ActiveModel = PoetryLineActiveModel;

    /// 根据非空正常数据更新
    fn update(&self, poetry_line: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            poetry_line.id = Set(*x);
        }

        if let Some(x) = &self.poetry_id {
            poetry_line.poetry_id = Set(*x);
        }

        if let Some(x) = &self.line_number {
            poetry_line.line_number = Set(*x);
        }

        if let Some(x) = &self.content {
            poetry_line.content = Set(String::from(x));
        }

        if let Some(x) = &self.pinyin {
            poetry_line.pinyin = Set(String::from(x));
        }

        if let Some(x) = &self.description {
            poetry_line.description = Set(String::from(x));
        }

        if let Some(x) = &self.notes {
            poetry_line.notes = Set(String::from(x));
        }

        if let Some(x) = &self.created_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                poetry_line.created_at = Set(x);
            }
        }

        if let Some(x) = &self.updated_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                poetry_line.updated_at = Set(Some(x));
            }
        } else {
            poetry_line.updated_at = Set(Some(utc_now()));
        }
    }

    ///
    /// 是创建操作需要再设置一些默认参数 如密码 uuid等
    /// 
    fn update_by_create(&self, poetry_line: &mut Self::ActiveModel) {
        if self.created_at.is_none() {
            poetry_line.created_at = Set(utc_now());
        }
    }
}

pub type CreatePoetryLineReqParams = PoetryLineReqParams;

///
/// 更新 poetry_line 参数
/// 
pub type UpdatePoetryLineReqParams = PoetryLineReqParams;
///
/// 删除数据参数
/// 
pub type DeletePoetryLineReqParams = PoetryLineReqParams;

