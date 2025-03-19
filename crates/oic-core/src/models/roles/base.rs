use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use oic_derives::{FilterParams, add_filter_fields};
use validator::Validate;
use crate::{
    models::RequestParamsUpdater,
    utils::utc_now,
    entities::prelude::*,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct RoleFilters {
    pub id: Option<i64>,
    pub vid: Option<String>,
}

/// 创建 Role 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct RoleReqParams {
    pub role_id: Option<i64>,
    #[validate(required(message = "必须指定 vid"), length(min = 2, message = "vid 最少2个字符"))]
    pub vid: Option<String>,
    #[validate(required(message = "必须指定 name"), length(min = 2, message = "name 最少2个字符"))]
    pub name: Option<String>,
    pub weight: Option<i32>,
    pub scope: Option<String>,
    #[validate(required(message = "必须指定 status"))]
    pub status: Option<String>,
    pub remark: Option<String>,
    #[serde(rename(deserialize = "createdAt"))]
    pub created_at: Option<DateTime>,
    #[serde(rename(deserialize = "updatedAt"))]
    pub updated_at: Option<DateTime>,
}

impl RequestParamsUpdater for RoleReqParams {
    type ActiveModel = RoleActiveModel;

    /// 根据非空正常数据更新
    fn update(&self, role: &mut Self::ActiveModel) {
        if let Some(x) = &self.role_id {
            role.role_id = Set(*x);
        }
        
        if let Some(x) = &self.vid {
            role.vid = Set(String::from(x));
        }

        if let Some(x) = &self.name {
            role.name = Set(String::from(x));
        }

        if let Some(x) = &self.weight {
            role.weight = Set(*x);
        }

        if let Some(x) = &self.scope {
            role.scope = Set(String::from(x));
        }

        if let Some(x) = &self.status {
            role.status = Set(String::from(x));
        }

        if let Some(x) = &self.remark {
            role.remark = Set(String::from(x));
        }

        if let Some(x) = &self.created_at {
            role.created_at = Set(*x);
        }

        if let Some(x) = &self.updated_at {
            role.updated_at = Set(Some(*x));
        } else {
            role.updated_at = Set(Some(utc_now()));
        }
    }

    ///
    /// 是创建操作需要再设置一些默认参数 如密码 uuid等
    /// 
    fn update_by_create(&self, role: &mut Self::ActiveModel) {
        role.role_id = ActiveValue::NotSet;


        if self.created_at.is_none() {
            role.created_at = Set(utc_now());
        }
    }
}

pub type CreateRoleReqParams = RoleReqParams;

///
/// 更新
/// 
pub type UpdateRoleReqParams = RoleReqParams;
///
/// 删除数据
/// 
pub type DeleteRoleReqParams = RoleReqParams;

