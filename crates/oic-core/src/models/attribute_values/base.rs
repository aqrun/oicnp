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
pub struct AttributeValueFilters {
    pub id: Option<i64>,
    pub vid: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub weight: Option<i16>,
    #[serde(rename(deserialize = "cssClass", serialize = "cssClass"))]
    pub css_class: Option<String>,
    #[serde(rename(deserialize = "listClass", serialize = "listClass"))]
    pub list_class: Option<String>,
    #[serde(rename(deserialize = "isDefault", serialize = "isDefault"))]
    pub is_default: Option<String>,
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

/// 创建 file 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct AttributeValueReqParams {
    pub id: Option<i64>,
    pub vid: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub weight: Option<i16>,
    #[serde(rename(deserialize = "cssClass", serialize = "cssClass"))]
    pub css_class: Option<String>,
    #[serde(rename(deserialize = "listClass", serialize = "listClass"))]
    pub list_class: Option<String>,
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

impl RequestParamsUpdater for AttributeValueReqParams {
    type ActiveModel = AttributeValueActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            item.id = Set(*x);
        }
        if let Some(x) = &self.vid {
            item.vid = Set(String::from(x));
        }
        if let Some(x) = &self.label {
            item.label = Set(String::from(x));
        }
        if let Some(x) = &self.value {
            item.value = Set(String::from(x));
        }
        if let Some(x) = &self.weight {
            item.weight = Set(*x);
        }
        if let Some(x) = &self.css_class {
            item.css_class = Set(String::from(x));
        }
        if let Some(x) = &self.list_class {
            item.list_class = Set(String::from(x));
        }
        if let Some(x) = &self.remark {
            item.remark = Set(String::from(x));
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

pub type CreateAttributeValueReqParams = AttributeValueReqParams;
///
/// 更新 attribute_value 参数
/// 
pub type UpdateAttributeValueReqParams = AttributeValueReqParams;
/// 删除数据参数
pub type DeleteAttributeValueReqParams = AttributeValueReqParams;
