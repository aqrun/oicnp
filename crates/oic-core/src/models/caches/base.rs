use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use loco_rs::prelude::*;
use crate::{
    RequestParamsUpdater,
    utils::utc_now,
    entities::prelude::*,
    constants::DATE_TIME_FORMAT,
};
use sea_orm::FromQueryResult;
use strum::EnumString;

#[derive(strum::Display, EnumString, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum CacheScope {
    Captcha,
    Session,
    Other,
}

#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct CacheScopeModel {
    pub scope: String,
    pub label: String,
}

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct CacheFilters {
    pub id: Option<i64>,
    #[serde(rename(deserialize = "cacheKey", serialize = "cacheKey"))]
    pub cache_key: Option<String>,
    #[serde(rename(deserialize = "cacheValue", serialize = "cacheValue"))]
    pub cache_value: Option<String>,
    pub scope: Option<String>,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "expiredAt", serialize = "expiredAt"))]
    pub expired_at: Option<String>,
}

/// 创建 file 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct CacheReqParams {
    pub id: Option<i64>,
    #[serde(rename(deserialize = "cacheKey", serialize = "cacheKey"))]
    pub cache_key: Option<String>,
    #[serde(rename(deserialize = "cacheValue", serialize = "cacheValue"))]
    pub cache_value: Option<String>,
    pub scope: Option<String>,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<String>,
    #[serde(rename(deserialize = "expiredAt", serialize = "expiredAt"))]
    pub expired_at: Option<String>,
}

impl RequestParamsUpdater for CacheReqParams {
    type ActiveModel = CacheActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            item.id = Set(*x);
        }
        if let Some(x) = &self.cache_key {
            item.cache_key = Set(String::from(x));
        }
        if let Some(x) = &self.cache_value {
            item.cache_value = Set(String::from(x));
        }
        if let Some(x) = &self.scope {
            item.scope = Set(String::from(x));
        }
        if let Some(x) = &self.expired_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                item.expired_at = Set(Some(x));
            }
        }
        if let Some(x) = &self.created_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                item.created_at = Set(x);
            }
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        item.id = ActiveValue::NotSet;

        if item.created_at.is_not_set() {
            item.created_at = Set(utc_now());
        }
    }
}

pub type CreateCacheReqParams = CacheReqParams;
///
/// 更新 note 参数
/// 
pub type UpdateCacheReqParams = CacheReqParams;
/// 删除数据参数
pub type DeleteCacheReqParams = CacheReqParams;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default, FromQueryResult)]
pub struct PartialCacheModel {
    pub id: Option<i64>,
    #[serde(rename(deserialize = "cacheKey", serialize = "cacheKey"))]
    pub cache_key: Option<String>,
    #[serde(rename(deserialize = "cacheValue", serialize = "cacheValue"))]
    pub cache_value: Option<String>,
    #[serde(rename(deserialize = "scope", serialize = "scope"))]
    pub scope: Option<String>,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<DateTime>,
    #[serde(rename(deserialize = "expiredAt", serialize = "expiredAt"))]
    pub expired_at: Option<DateTime>,
}
