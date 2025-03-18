use serde::{Deserialize, Serialize};
use validator::Validate;
use oic_derives::{add_filter_fields, FilterParams};
use crate::RequestParamsUpdater;
use crate::entities::prelude::MenuActiveModel;
use crate::utils::utc_now;
use loco_rs::prelude::Set;

#[add_filter_fields]
#[derive(FilterParams, Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct MenuFilters {
    pub id: Option<i32>,
    pub mid: Option<String>,
    pub pid: Option<String>,
    pub name: Option<String>,
    pub depth: Option<String>,
}

/// 创建 note 参数
#[derive(Deserialize, Serialize, Debug, Validate, Default)]
#[serde(default)]
pub struct MenuReqParams {
    pub id: Option<i32>,
    pub mid: Option<String>,
    pub pid: Option<String>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub weight: Option<i32>,
    pub api: Option<String>,
    pub status: Option<String>,
    pub visible: Option<String>,
    #[serde(rename(deserialize = "isCache"))]
    pub is_cache: Option<String>,
    #[serde(rename(deserialize = "isFrame"))]
    pub is_frame: Option<String>,
    pub remark: Option<String>,
}

impl RequestParamsUpdater for MenuReqParams {
    type ActiveModel = MenuActiveModel;

    fn update(&self, item: &mut Self::ActiveModel) {
        if let Some(x) = &self.mid {
            item.mid = Set(String::from(x));
        }

        if let Some(x) = &self.pid {
            item.pid = Set(String::from(x));
        }

        if let Some(x) = &self.path {
            item.path = Set(String::from(x));
        }

        if let Some(x) = &self.name {
            item.name = Set(String::from(x));
        }

        if let Some(x) = &self.icon {
            item.icon = Set(String::from(x));
        }

        if let Some(x) = &self.weight {
            item.weight = Set(*x);
        }

        if let Some(x) = &self.api {
            item.api = Set(String::from(x));
        }

        if let Some(x) = &self.status {
            item.status = Set(String::from(x));
        }

        if let Some(x) = &self.visible {
            item.visible = Set(String::from(x));
        }

        if let Some(x) = &self.is_cache {
            item.is_cache = Set(String::from(x));
        }

        if let Some(x) = &self.is_frame {
            item.is_frame = Set(String::from(x));
        }

        if let Some(x) = &self.remark {
            item.remark = Set(String::from(x));
        }
    }

    fn update_by_create(&self, item: &mut Self::ActiveModel) {
        if item.created_at.is_not_set() {
            item.created_at = Set(utc_now());
        }
    }
}

pub type CreateMenuReqParams = MenuReqParams;
pub type UpdateMenuReqParams = MenuReqParams;
pub type DeleteMenuReqParams = MenuReqParams;

/**
 * 树结构返回的菜单数据
 */
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct MenuTreeItem {
    pub id: i32,
    pub mid: String,
    pub key: String,
    pub pid: String,
    pub path: String,
    pub label: String,
    pub weight: i32,
    pub icon: String,
    pub children: Vec<MenuTreeItem>,
}
