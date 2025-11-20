use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::utils::utc_now;
use crate::{
    RequestParamsUpdater, uuid,
    constants::DATE_TIME_FORMAT,
};

pub use crate::entities::poetry::{
  AuthorActiveModel,
  AuthorEntity,
  AuthorModel,
  AuthorColumn,
};


#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct AuthorFilters {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename(deserialize = "birthAt"))]
    pub birth_at: Option<String>,
    #[serde(rename(deserialize = "deathAt"))]
    pub death_at: Option<String>,
    pub dynasty: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

/// 创建 author 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct AuthorReqParams {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename(deserialize = "birthAt"))]
    pub birth_at: Option<String>,
    #[serde(rename(deserialize = "deathAt"))]
    pub death_at: Option<String>,
    pub dynasty: Option<String>,
    pub weight: Option<i32>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<String>,
}

impl RequestParamsUpdater for AuthorReqParams {
    type ActiveModel = AuthorActiveModel;

    /// 根据非空正常数据更新
    fn update(&self, author: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            author.id = Set(*x);
        }

        if let Some(x) = &self.uuid {
            author.uuid = Set(String::from(x));
        }

        if let Some(x) = &self.name {
            author.name = Set(String::from(x));
        }

        if let Some(x) = &self.description {
            author.description = Set(String::from(x));
        }

        if let Some(x) = &self.birth_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                author.birth_at = Set(x);
            }
        }

        if let Some(x) = &self.death_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                author.death_at = Set(x);
            }
        }

        if let Some(x) = &self.dynasty {
            author.dynasty = Set(String::from(x));
        }

        if let Some(x) = &self.weight {
            author.weight = Set(*x);
        }

        if let Some(x) = &self.created_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                author.created_at = Set(x);
            }
        }

        if let Some(x) = &self.updated_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                author.updated_at = Set(Some(x));
            }
        } else {
            author.updated_at = Set(Some(utc_now()));
        }
    }

    ///
    /// 是创建操作需要再设置一些默认参数 如密码 uuid等
    /// 
    fn update_by_create(&self, author: &mut Self::ActiveModel) {
        if self.uuid.is_none() {
            author.uuid = Set(uuid!());
        }

        if self.created_at.is_none() {
            author.created_at = Set(utc_now());
        }
    }
}

pub type CreateAuthorReqParams = AuthorReqParams;

///
/// 更新 author 参数
/// 
pub type UpdateAuthorReqParams = AuthorReqParams;
///
/// 删除数据参数
/// 
pub type DeleteAuthorReqParams = AuthorReqParams;

