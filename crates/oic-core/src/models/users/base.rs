use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::utils::default_string;

pub use crate::entities::prelude::{
  UserActiveModel,
  UserEntity,
  UserModel,
  UserColumn,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterParams {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub name: String,
    #[validate(custom(function = "validation::is_valid_email"))]
    pub email: String,
}

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug)]
pub struct UserFilters {
    pub uid: Option<i64>,
    pub uuid: Option<String>,
}

/// 创建 User 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
pub struct CreateUserReqParams {
    #[serde(default = "default_string")]
    pub uuid: String,
    #[validate(required(message = "必须指定 username"), length(min = 2, message = "username 最少2个字符"))]
    pub username: Option<String>,
    pub nickname: Option<String>,
    #[validate(required(message = "必须指定 password"), length(min = 2, message = "password 最少6个字符"))]
    pub password: Option<String>,
    #[validate(email(message = "邮箱地址不合法"))]
    pub email: Option<String>,
    #[serde(default = "default_string")]
    pub status: String,
    #[serde(default = "default_string")]
    #[serde(rename(deserialize = "isAdmin"))]
    pub is_admin: String,
}

///
/// 更新 note 参数
/// 
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct UpdateUserReqParams {
    pub uid: Option<i64>,
    #[validate(required(message = "必须指定 username"), length(min = 2, message = "username 最少2个字符"))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

/// 删除数据参数
#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct DeleteUserReqParams {
    pub uid: Option<i64>,
}
