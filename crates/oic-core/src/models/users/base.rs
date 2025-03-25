use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::utils::{
    utc_now,
    encrypt_password,
    generate_salt,
    uuid as getUuid,
};
use crate::RequestParamsUpdater;

pub use crate::entities::prelude::{
  UserActiveModel,
  UserEntity,
  UserModel,
  UserColumn,
};

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(default)]
pub struct LoginParams {
    #[validate(length(min = 1, message = "必须指定 email 或 password"))]
    pub email: String,
    #[validate(length(min = 1, message = "必须指定 email 或 password"))]
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
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct UserReqParams {
    pub uid: Option<i64>,
    pub uuid: Option<String>,
    #[validate(required(message = "必须指定 username"), length(min = 2, message = "username 最少2个字符"))]
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    #[serde(rename(deserialize = "apiKey"))]
    pub api_key: Option<String>,
    pub reset_token: Option<String>,
    pub reset_sent_at: Option<DateTime>,
    pub status: Option<String>,
    #[validate(email(message = "邮箱地址不合法"))]
    pub email: Option<String>,
    pub gender: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename(deserialize = "roleId"))]
    pub role_id: Option<i64>,
    pub roles: Option<Vec<String>>,
    #[serde(rename(deserialize = "dptId"))]
    pub dpt_id: Option<i64>,
    pub remark: Option<String>,
    #[serde(rename(deserialize = "isAdmin"))]
    pub is_admin: Option<String>,
    pub phone: Option<String>,
    #[serde(rename(deserialize = "createdBy"))]
    pub created_by: Option<i64>,
    #[serde(rename(deserialize = "updatedBy"))]
    pub updated_by: Option<i64>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<DateTime>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<DateTime>,
    #[serde(rename(deserialize = "deletedAt"))]
    pub deleted_at: Option<DateTime>,
}

impl RequestParamsUpdater for UserReqParams {
    type ActiveModel = UserActiveModel;

    /// 根据非空正常数据更新
    fn update(&self, user: &mut Self::ActiveModel) {
        if let Some(x) = &self.uuid {
            user.uuid = Set(String::from(x));
        }

        if let Some(x) = &self.username {
            user.username = Set(String::from(x));
        }

        if let Some(x) = &self.nickname {
            user.nickname = Set(String::from(x));
        }

        if let Some(x) = &self.salt {
            user.salt = Set(String::from(x));
        }

        if let Some(x) = &self.api_key {
            user.api_key = Set(String::from(x));
        }

        if let Some(x) = &self.reset_token {
            user.reset_token = Set(String::from(x));
        }

        if let Some(x) = &self.reset_sent_at {
            user.reset_sent_at = Set(Some(*x));
        }

        if let Some(x) = &self.status {
            user.status = Set(String::from(x));
        }

        if let Some(x) = &self.email {
            user.email = Set(String::from(x));
        }

        if let Some(x) = &self.gender {
            user.gender = Set(String::from(x));
        }

        if let Some(x) = &self.avatar {
            user.avatar = Set(String::from(x));
        }

        if let Some(x) = &self.role_id {
            user.role_id = Set(*x);
        }

        if let Some(x) = &self.dpt_id {
            user.dpt_id = Set(*x);
        }

        if let Some(x) = &self.remark {
            user.remark = Set(String::from(x));
        }

        if let Some(x) = &self.is_admin {
            user.is_admin = Set(String::from(x));
        }

        if let Some(x) = &self.phone {
            user.phone = Set(String::from(x));
        }

        if let Some(x) = &self.created_by {
            user.created_by = Set(*x);
        }

        if let Some(x) = &self.updated_by {
            user.updated_by = Set(*x);
        }

        if let Some(x) = &self.created_at {
            user.created_at = Set(*x);
        }

        if let Some(x) = &self.updated_at {
            user.updated_at = Set(Some(*x));
        } else {
            user.updated_at = Set(Some(utc_now()));
        }

        if let Some(x) = &self.deleted_at {
            user.deleted_at = Set(Some(*x));
        }
    }

    ///
    /// 是创建操作需要再设置一些默认参数 如密码 uuid等
    /// 
    fn update_by_create(&self, user: &mut Self::ActiveModel) {
        if self.uuid.is_none() {
            user.uuid = Set(getUuid());
        }
        
        let mut password = String::from("123456");

        if let Some(x) = &self.password {
            password = String::from(x);
        }

        // 存在盐值直接使用已生成的密码
        if let Some(x) = &self.salt {
            user.salt = Set(String::from(x));
            user.password = Set(password);
        } else {
            // 不存在盐址新生成加密字符串
            let salt = generate_salt();
            user.password = Set(encrypt_password(salt.as_str(), password.as_str()));
            user.salt = Set(salt);
        }
        
        if self.created_at.is_none() {
            user.created_at = Set(utc_now());
        }
    }
}

pub type CreateUserReqParams = UserReqParams;

///
/// 更新 note 参数
/// 
pub type UpdateUserReqParams = UserReqParams;
///
/// 删除数据参数
/// 
pub type DeleteUserReqParams = UserReqParams;

