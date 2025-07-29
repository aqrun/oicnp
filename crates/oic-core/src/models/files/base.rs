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
pub struct FileFilters {
    #[serde(rename(deserialize = "fileId", serialize = "fileId"))]
    pub file_id: Option<i64>,
    pub uid: Option<i64>,
    pub filename: Option<String>,
    pub uri: Option<String>,
    pub storage: Option<String>,
    pub mime: Option<String>,
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

/// 创建 file 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct FileReqParams {
    #[serde(rename(deserialize = "fileId", serialize = "fileId"))]
    pub file_id: Option<i64>,
    pub uid: Option<i64>,
    pub filename: Option<String>,
    pub uri: Option<String>,
    pub storage: Option<String>,
    pub mime: Option<String>,
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
    #[serde(rename(deserialize = "createdByUsername", serialize = "createdByUsername"))]
    pub created_by_username: Option<String>,
}

impl RequestParamsUpdater for FileReqParams {
    type ActiveModel = FileActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.file_id {
            item.file_id = Set(*x);
        }
        if let Some(x) = &self.uid {
            item.uid = Set(*x);
        }
        if let Some(x) = &self.filename {
            item.filename = Set(String::from(x));
        }
        if let Some(x) = &self.uri {
            item.uri = Set(String::from(x));
        }
        if let Some(x) = &self.storage {
            item.storage = Set(String::from(x));
        }
        if let Some(x) = &self.mime {
            item.mime = Set(String::from(x));
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
        item.file_id = ActiveValue::NotSet;

        if item.created_at.is_not_set() {
            item.created_at = Set(utc_now());
        }
    }
}

pub type CreateFileReqParams = FileReqParams;
///
/// 更新 note 参数
/// 
pub type UpdateFileReqParams = FileReqParams;
/// 删除数据参数
pub type DeleteFileReqParams = FileReqParams;

#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct UploadFileRes {
    pub id: i64,
    pub name: String,
    pub size: i64,
    #[serde(rename(deserialize = "fileType", serialize = "fileType"))]
    pub file_type: String,
    pub url: String,
    pub mime: String,
    pub status: String,
}