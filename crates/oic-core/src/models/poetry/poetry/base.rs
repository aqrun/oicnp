use loco_rs::prelude::*;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::utils::utc_now;
use crate::{RequestParamsUpdater, uuid, constants::DATE_TIME_FORMAT};
use crate::entities::poetry::ChapterModel;

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
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub author_id: Option<i32>,
    pub dynasty: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(serialize = "hotWeight", deserialize = "hotWeight"))]
    pub hot_weight: Option<i32>,
    pub content: Option<String>,
    #[serde(rename(serialize = "wordCount", deserialize = "wordCount"))]
    pub word_count: Option<i32>,
    pub tags: Option<String>,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(serialize = "updatedAt", deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
    #[serde(rename(serialize = "poetryAmount", deserialize = "poetryAmount"))]
    pub poetry_amount: Option<u64>,
    #[serde(rename(serialize = "chapterAmount", deserialize = "chapterAmount"))]
    pub chapter_amount: Option<u64>,
}

/// 创建 poetry 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct PoetryReqParams {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub title: Option<String>,
    #[serde(rename(serialize = "authorId", deserialize = "authorId"))]
    pub author_id: Option<i32>,
    pub dynasty: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(serialize = "hotWeight", deserialize = "hotWeight"))]
    pub hot_weight: Option<i32>,
    pub content: Option<String>,
    #[serde(rename(serialize = "wordCount", deserialize = "wordCount"))]
    pub word_count: Option<i32>,
    pub tags: Option<String>,
    /// 诗词说明
    pub description: Option<String>,
    #[serde(rename(serialize = "createdAt", deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(serialize = "updatedAt", deserialize = "updatedAt"))]
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
            poetry.hot_weight = Set(*x);
        }

        if let Some(x) = &self.content {
            poetry.content = Set(String::from(x));
        }

        if let Some(x) = &self.word_count {
            poetry.word_count = Set(*x);
        }

        if let Some(x) = &self.tags {
            poetry.tags = Set(String::from(x));
        }

        if let Some(x) = &self.description {
            poetry.description = Set(String::from(x));
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

///
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct PoetryAnalysisView {
    /// 总诗词数量
    pub total_poetry: u64,
    /// 总作者数量
    pub total_author: u64,
    /// 总文言文数量
    pub total_wen_yan_wen: u64,
    /// 总字数
    pub total_word_count: u64,
}

impl std::fmt::Display for PoetryAnalysisView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"诗词数量:    {}
作者数量:    {}
文言文数量:  {}
总文字数量:  {}
"#,
            self.total_poetry,
            self.total_author,
            self.total_wen_yan_wen,
            self.total_word_count
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, FromQueryResult)]
#[serde(default)]
pub struct CountDataModel {
    pub total_word_count: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, FromQueryResult)]
#[serde(default)]
pub struct PoetryListDataModel {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    #[serde(rename(deserialize = "authorId", serialize = "authorId"))]
    pub author_id: i32,
    pub dynasty: String,
    pub weight: i32,
    #[serde(rename(deserialize = "hotWeight", serialize = "hotWeight"))]
    pub hot_weight: i32,
    pub content: String,
    #[serde(rename(deserialize = "wordCount", serialize = "wordCount"))]
    pub word_count: i32,
    pub tags: String,
    /// 诗词说明
    pub description: String,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt", serialize = "updatedAt"))]
    pub updated_at: Option<String>,
    #[serde(rename(deserialize = "authorUuid", serialize = "authorUuid"))]
    pub author_uuid: Option<String>,
    #[serde(rename(deserialize = "authorName", serialize = "authorName"))]
    pub author_name: Option<String>,
    #[serde(rename(deserialize = "isBook", serialize = "isBook"))]
    pub is_book: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct PoetryListPageDataResponse {
    pub poetry_list: Vec<PoetryListDataModel>,
    pub chapter_list: Vec<ChapterModel>,
    pub total: u64,
    pub page: u64,
    #[serde(rename(deserialize = "pageSize", serialize = "pageSize"))]
    pub page_size: u64,
}
