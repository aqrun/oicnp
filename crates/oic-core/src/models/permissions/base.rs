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
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct PermissionFilters {
    pub id: Option<i64>,
    pub vid: Option<String>,
}

/// 创建 Role 参数
#[derive(Deserialize, Serialize, Debug, Validate, Clone, Default)]
#[serde(default)]
pub struct PermissionReqParams {
    #[serde(rename(deserialize = "permissionId"))]
    pub permission_id: Option<i64>,
    #[validate(required(message = "必须指定 vid"), length(min = 2, message = "vid 最少2个字符"))]
    pub vid: Option<String>,
    #[serde(rename(deserialize = "parentVid"))]
    pub parent_vid: Option<String>,
    pub pid: Option<i64>,
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

impl RequestParamsUpdater for PermissionReqParams {
    type ActiveModel = PermissionActiveModel;

    /// 根据非空正常数据更新
    fn update(&self, permission: &mut Self::ActiveModel) {
        if let Some(x) = &self.permission_id {
            permission.permission_id = Set(*x);
        }
        
        if let Some(x) = &self.vid {
            permission.vid = Set(String::from(x));
        }

        if let Some(x) = &self.name {
            permission.name = Set(String::from(x));
        }

        if let Some(x) = &self.weight {
            permission.weight = Set(*x);
        }

        if let Some(x) = &self.scope {
            permission.scope = Set(String::from(x));
        }

        if let Some(x) = &self.status {
            permission.status = Set(String::from(x));
        }

        if let Some(x) = &self.remark {
            permission.remark = Set(String::from(x));
        }

        if let Some(x) = &self.created_at {
            permission.created_at = Set(*x);
        }

        if let Some(x) = &self.updated_at {
            permission.updated_at = Set(Some(*x));
        } else {
            permission.updated_at = Set(Some(utc_now()));
        }
    }

    ///
    /// 是创建操作需要再设置一些默认参数 如密码 uuid等
    /// 
    fn update_by_create(&self, permission: &mut Self::ActiveModel) {
        permission.permission_id = ActiveValue::NotSet;


        if self.created_at.is_none() {
            permission.created_at = Set(utc_now());
        }
    }
}

pub type CreatePermissionReqParams = PermissionReqParams;

///
/// 更新
/// 
pub type UpdatePermissionReqParams = PermissionReqParams;
///
/// 删除数据
/// 
pub type DeletePermissionReqParams = PermissionReqParams;

