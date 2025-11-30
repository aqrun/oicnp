use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use crate::{
    models::RequestParamsUpdater,
    entities::prelude::*,
    constants::DATE_TIME_FORMAT,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct LoginLogFilters {
    pub id: Option<i64>,
    #[serde(rename(deserialize = "loginName", serialize = "loginName"))]
    pub login_name: Option<String>,
    pub net: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub device: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub module: Option<String>,
    #[serde(rename(deserialize = "loginAt", serialize = "loginAt"))]
    pub login_at: Option<String>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct LoginLogReqParams {
    pub id: Option<i64>,
    #[serde(rename(deserialize = "loginName", serialize = "loginName"))]
    pub login_name: Option<String>,
    pub net: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub device: Option<String>,
    pub status: Option<String>,
    pub message: Option<String>,
    pub module: Option<String>,
    #[serde(rename(deserialize = "loginAt", serialize = "loginAt"))]
    pub login_at: Option<String>,
}

impl RequestParamsUpdater for LoginLogReqParams {
    type ActiveModel = LoginLogActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            item.id = Set(*x);
        }
        if let Some(x) = &self.login_name {
            item.login_name = Set(String::from(x));
        }
        if let Some(x) = &self.net {
            item.net = Set(String::from(x));
        }
        if let Some(x) = &self.ip {
            item.ip = Set(String::from(x));
        }
        if let Some(x) = &self.location {
            item.location = Set(String::from(x));
        }
        if let Some(x) = &self.browser {
            item.browser = Set(String::from(x));
        }
        if let Some(x) = &self.os {
            item.os = Set(String::from(x));
        }
        if let Some(x) = &self.device {
            item.device = Set(String::from(x));
        }
        if let Some(x) = &self.status {
            item.status = Set(String::from(x));
        }
        if let Some(x) = &self.message {
            item.message = Set(String::from(x));
        }
        if let Some(x) = &self.module {
            item.module = Set(String::from(x));
        }
        if let Some(x) = &self.login_at {
            if let Ok(x) = DateTime::parse_from_str(x, DATE_TIME_FORMAT) {
                item.login_at = Set(Some(x));
            }
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        item.id = ActiveValue::NotSet;
    }
}

pub type CreateLoginLogReqParams = LoginLogReqParams;
pub type UpdateLoginLogReqParams = LoginLogReqParams;
pub type DeleteLoginLogReqParams = LoginLogReqParams;

