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
pub struct OperationLogFilters {
    pub id: Option<i64>,
    #[serde(rename(deserialize = "timeId", serialize = "timeId"))]
    pub time_id: Option<i64>,
    pub title: Option<String>,
    #[serde(rename(deserialize = "businessType", serialize = "businessType"))]
    pub business_type: Option<String>,
    pub method: Option<String>,
    #[serde(rename(deserialize = "requestMethod", serialize = "requestMethod"))]
    pub request_method: Option<String>,
    #[serde(rename(deserialize = "operatorType", serialize = "operatorType"))]
    pub operator_type: Option<String>,
    pub name: Option<String>,
    #[serde(rename(deserialize = "departmentName", serialize = "departmentName"))]
    pub department_name: Option<String>,
    pub url: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub param: Option<String>,
    #[serde(rename(deserialize = "pathParam", serialize = "pathParam"))]
    pub path_param: Option<String>,
    #[serde(rename(deserialize = "jsonResult", serialize = "jsonResult"))]
    pub json_result: Option<String>,
    pub status: Option<String>,
    #[serde(rename(deserialize = "errorMessage", serialize = "errorMessage"))]
    pub error_message: Option<String>,
    pub duration: Option<i64>,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<String>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct OperationLogReqParams {
    pub id: Option<i64>,
    pub title: Option<String>,
    #[serde(rename(deserialize = "businessType", serialize = "businessType"))]
    pub business_type: Option<String>,
    pub method: Option<String>,
    #[serde(rename(deserialize = "requestMethod", serialize = "requestMethod"))]
    pub request_method: Option<String>,
    #[serde(rename(deserialize = "operatorType", serialize = "operatorType"))]
    pub operator_type: Option<String>,
    pub name: Option<String>,
    #[serde(rename(deserialize = "departmentName", serialize = "departmentName"))]
    pub department_name: Option<String>,
    pub url: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub param: Option<String>,
    #[serde(rename(deserialize = "pathParam", serialize = "pathParam"))]
    pub path_param: Option<String>,
    #[serde(rename(deserialize = "jsonResult", serialize = "jsonResult"))]
    pub json_result: Option<String>,
    pub status: Option<String>,
    #[serde(rename(deserialize = "errorMessage", serialize = "errorMessage"))]
    pub error_message: Option<String>,
    pub duration: Option<i64>,
    #[serde(rename(deserialize = "createdAt", serialize = "createdAt"))]
    pub created_at: Option<String>,
}

impl RequestParamsUpdater for OperationLogReqParams {
    type ActiveModel = OperationLogActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            item.id = Set(*x);
        }
        if let Some(x) = &self.title {
            item.title = Set(String::from(x));
        }
        if let Some(x) = &self.business_type {
            item.business_type = Set(String::from(x));
        }
        if let Some(x) = &self.method {
            item.method = Set(String::from(x));
        }
        if let Some(x) = &self.request_method {
            item.request_method = Set(String::from(x));
        }
        if let Some(x) = &self.operator_type {
            item.operator_type = Set(String::from(x));
        }
        if let Some(x) = &self.name {
            item.name = Set(String::from(x));
        }
        if let Some(x) = &self.department_name {
            item.department_name = Set(String::from(x));
        }
        if let Some(x) = &self.url {
            item.url = Set(String::from(x));
        }
        if let Some(x) = &self.ip {
            item.ip = Set(String::from(x));
        }
        if let Some(x) = &self.location {
            item.location = Set(String::from(x));
        }
        if let Some(x) = &self.param {
            item.param = Set(String::from(x));
        }
        if let Some(x) = &self.path_param {
            item.path_param = Set(String::from(x));
        }
        if let Some(x) = &self.json_result {
            item.json_result = Set(String::from(x));
        }
        if let Some(x) = &self.status {
            item.status = Set(String::from(x));
        }
        if let Some(x) = &self.error_message {
            item.error_message = Set(String::from(x));
        }
        if let Some(x) = &self.duration {
            item.duration = Set(*x);
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

pub type CreateOperationLogReqParams = OperationLogReqParams;
pub type UpdateOperationLogReqParams = OperationLogReqParams;
pub type DeleteOperationLogReqParams = OperationLogReqParams;

