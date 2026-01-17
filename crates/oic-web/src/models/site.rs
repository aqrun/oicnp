use serde::{Deserialize, Serialize};
use chrono::{Utc, Datelike};
use oic_core::{
  entities::prelude::*,
  models::nodes::NodeDetailModel,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SiteConfig {
  pub title: String,
  pub slogan: String,
  pub description: String,
  pub keywords: String,
  pub url: String,
  pub menus: Vec<SiteMenu>,
  pub blog_categories: Vec<BlogCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SiteMenu {
  pub name: String,
  pub vid: String,
  pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct BlogCategory {
  pub name: String,
  pub vid: String,
  pub href: String,
}

pub fn get_current_year() -> i32 {
  Utc::now().year()
}

#[derive(Debug, Default)]
pub struct SideBar {
  pub has_calendar: bool,
  pub has_recommend_blogs: bool,
  pub recommend_blogs: Vec<NodeDetailModel>,
  pub has_blog_tags: bool,
  pub blog_tags: Vec<TagModel>,
}