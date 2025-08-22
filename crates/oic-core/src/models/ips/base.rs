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

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct IpFilters {
    pub id: Option<i64>,
    pub ip: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    #[serde(rename(deserialize = "provinceCode", serialize = "provinceCode"))]
    pub province_code: Option<String>,
    #[serde(rename(deserialize = "cityCode", serialize = "cityCode"))]
    pub city_code: Option<String>,
    pub region: Option<String>,
    #[serde(rename(deserialize = "regionCode", serialize = "regionCode"))]
    pub region_code: Option<String>,
    #[serde(rename(deserialize = "regionNames", serialize = "regionNames"))]
    pub region_names: Option<String>,
    pub network: Option<String>,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<String>,
}

/// 创建 file 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct IpReqParams {
    pub id: Option<i64>,
    pub ip: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    #[serde(rename(deserialize = "provinceCode", serialize = "provinceCode"))]
    pub province_code: Option<String>,
    #[serde(rename(deserialize = "cityCode", serialize = "cityCode"))]
    pub city_code: Option<String>,
    pub region: Option<String>,
    #[serde(rename(deserialize = "regionCode", serialize = "regionCode"))]
    pub region_code: Option<String>,
    #[serde(rename(deserialize = "regionNames", serialize = "regionNames"))]
    pub region_names: Option<String>,
    pub network: Option<String>,
    pub created_at: Option<String>,
}

impl RequestParamsUpdater for IpReqParams {
    type ActiveModel = IpActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            item.id = Set(*x);
        }
        if let Some(x) = &self.ip {
            item.ip = Set(String::from(x));
        }
        if let Some(x) = &self.province {
            item.province = Set(String::from(x));
        }
        if let Some(x) = &self.province_code {
            item.province_code = Set(String::from(x));
        }
        if let Some(x) = &self.city {
            item.city = Set(String::from(x));
        }
        if let Some(x) = &self.city_code {
            item.city_code = Set(String::from(x));
        }
        if let Some(x) = &self.region {
            item.region = Set(String::from(x));
        }
        if let Some(x) = &self.region_code {
            item.region_code = Set(String::from(x));
        }
        if let Some(x) = &self.region_names {
            item.region_names = Set(String::from(x));
        }
        if let Some(x) = &self.network {
            item.network = Set(String::from(x));
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

pub type CreateIpReqParams = IpReqParams;
///
/// 更新 note 参数
/// 
pub type UpdateIpReqParams = IpReqParams;
/// 删除数据参数
pub type DeleteIpReqParams = IpReqParams;
