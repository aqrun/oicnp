use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use crate::{
    models::RequestParamsUpdater,
    utils::utc_now,
    entities::prelude::*,
    constants::DATE_TIME_FORMAT,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct DepartmentFilters {
    pub id: Option<i64>,
    pub pid: Option<i64>,
    pub name: Option<String>,
    pub weight: Option<i16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
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
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct DepartmentReqParams {
    pub id: Option<i64>,
    pub pid: Option<i64>,
    #[validate(required(message = "必须指定 name"), length(min = 2, message = "name 最少2个字符"))]
    pub name: Option<String>,
    pub weight: Option<i16>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
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
}

impl RequestParamsUpdater for DepartmentReqParams {
    type ActiveModel = DepartmentActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            item.id = Set(*x);
        }
        if let Some(x) = &self.pid {
            item.pid = Set(*x);
        }
        if let Some(x) = &self.name {
            item.name = Set(String::from(x));
        }
        if let Some(x) = &self.weight {
            item.weight = Set(*x);
        }
        if let Some(x) = &self.leader {
            item.leader = Set(String::from(x));
        }
        if let Some(x) = &self.phone {
            item.phone = Set(String::from(x));
        }
        if let Some(x) = &self.email {
            item.email = Set(String::from(x));
        }
        if let Some(x) = &self.status {
            item.status = Set(String::from(x));
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
        item.id = ActiveValue::NotSet;

        if item.created_at.is_not_set() {
            item.created_at = Set(utc_now());
        }
    }
}

pub type CreateDepartmentReqParams = DepartmentReqParams;
pub type UpdateDepartmentReqParams = DepartmentReqParams;
pub type DeleteDepartmentReqParams = DepartmentReqParams;

