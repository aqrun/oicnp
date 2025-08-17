use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use crate::{
    models::RequestParamsUpdater,
    entities::prelude::*,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct UserOnlineFilters {
    pub uid: Option<i64>,
    #[serde(rename(deserialize = "tokenId", serialize = "tokenId"))]
    pub token_id: Option<String>,
    #[serde(rename(deserialize = "tokenExpire", serialize = "tokenExpire"))]
    pub token_expire: Option<i64>,
    #[serde(rename(deserialize = "loginAt", serialize = "loginAt"))]
    pub login_at: Option<DateTime>,
    pub username: Option<String>,
    #[serde(rename(deserialize = "dptName", serialize = "dptName"))]
    pub dpt_name: Option<String>,
    pub net: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub device: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct UserOnlineReqParams {
    pub uid: Option<i64>,
    #[validate(required(message = "必须指定 tokenId"), length(min = 2, message = "tokenId 最少2个字符"))]
    pub token_id: Option<String>,
    #[serde(rename(deserialize = "tokenExpire", serialize = "tokenExpire"))]
    pub token_expire: Option<i64>,
    #[serde(rename(deserialize = "loginAt", serialize = "loginAt"))]
    pub login_at: Option<DateTime>,
    pub username: Option<String>,
    #[serde(rename(deserialize = "dptName", serialize = "dptName"))]
    pub dpt_name: Option<String>,
    pub net: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub device: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
}
impl RequestParamsUpdater for UserOnlineReqParams {
    type ActiveModel = UserOnlineActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.uid {
            item.uid = Set(*x);
        }
        if let Some(x) = &self.token_id {
            item.token_id = Set(String::from(x));
        }
        if let Some(x) = &self.token_expire {
            item.token_expire = Set(*x);
        }
        if let Some(x) = &self.login_at {
            item.login_at = Set(*x);
        }
        if let Some(x) = &self.username {
            item.username = Set(String::from(x));
        }
        if let Some(x) = &self.dpt_name {
            item.dpt_name = Set(String::from(x));
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        item.uid = ActiveValue::NotSet;
    }
}

pub type CreateUserOnlineReqParams = UserOnlineReqParams;
pub type UpdateUserOnlineReqParams = UserOnlineReqParams;
pub type DeleteUserOnlineReqParams = UserOnlineReqParams;

