use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use loco_rs::prelude::*;
use crate::{
    RequestParamsUpdater,
    utils::utc_now,
    entities::prelude::*,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default, Clone)]
#[serde(default)]
pub struct FileFilters {
    pub file_id: Option<i64>,
    pub uid: Option<i64>,
    pub filename: Option<String>,
    pub uri: Option<String>,
    pub storage: Option<String>,
    pub mime: Option<String>,
    pub status: Option<String>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

/// 创建 file 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct FileReqParams {
    pub file_id: Option<i64>,
    pub uid: Option<i64>,
    pub filename: Option<String>,
    pub uri: Option<String>,
    pub storage: Option<String>,
    pub mime: Option<String>,
    pub status: Option<String>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
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
            item.created_at = Set(*x);
        }
        if let Some(x) = &self.updated_at {
            item.updated_at = Set(Some(*x));
        } else {
            item.updated_at = Set(Some(utc_now()));
        }
        if let Some(x) = &self.deleted_at {
            item.deleted_at = Set(Some(*x));
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
