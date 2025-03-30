use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use crate::{
    models::RequestParamsUpdater,
    utils::utc_now,
    entities::prelude::*,
};

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct NoteFilters {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default, Clone)]
#[serde(default)]
pub struct NoteReqParams {
    pub id: Option<i64>,
    #[validate(required(message = "必须指定 title"), length(min = 2, message = "title 最少2个字符"))]
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl RequestParamsUpdater for NoteReqParams {
    type ActiveModel = NoteActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.id {
            item.id = Set(*x);
        }
        if let Some(x) = &self.title {
            item.title = Set(String::from(x));
        }
        if let Some(x) = &self.content {
            item.content = Set(String::from(x));
        }
        if let Some(x) = &self.created_at {
            item.created_at = Set(*x);
        }
        if let Some(x) = &self.updated_at {
            item.updated_at = Set(Some(*x));
        } else {
            item.updated_at = Set(Some(utc_now()));
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        item.id = ActiveValue::NotSet;

        if item.created_at.is_not_set() {
            item.created_at = Set(utc_now());
        }
    }
}

pub type CreateNoteReqParams = NoteReqParams;
pub type UpdateNoteReqParams = NoteReqParams;
pub type DeleteNoteReqParams = NoteReqParams;

