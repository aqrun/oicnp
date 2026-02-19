use serde::{Deserialize, Serialize};
use chrono::{Utc, Datelike};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SiteConfig {
  pub title: String,
  pub slogan: String,
  pub description: String,
  pub keywords: String,
  pub url: String,
  pub menus: Vec<SiteMenu>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SiteMenu {
  pub name: String,
  pub vid: String,
  pub href: String,
}
