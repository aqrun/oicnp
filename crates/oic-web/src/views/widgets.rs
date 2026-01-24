use askama::Template;
use crate::services::{
  get_cached_or_render,
  CacheConfig,
  describe_node_list,
  describe_tag_list,
};
use crate::models::{RenderBytes, SideNavItem};
use crate::WebAppContext;
use bytes::Bytes;
use oic_core::{
  entities::prelude::*,
  models::{
    nodes::{NodeDetailModel, NodeFilters},
    tags::TagFilters,
  },
};
use oic_core::typings::JsonResPayload;

#[derive(Template)]
#[template(path = "widgets/calendar.html")]
pub struct CalendarWidget {
  pub id: String,
  pub class: String,
}

impl Default for CalendarWidget {
  fn default() -> Self {
    Self {
      id: "side-bar-calendar".to_string(),
      class: "side-bar-widget".to_string(),
    }
  }
}

impl CalendarWidget {
  pub async fn get_html(&self, ctx: &WebAppContext) -> String {
    let html = match get_cached_or_render(
      &ctx.cache,
      format!("widget:{}", self.id).as_str(),
      || async {
        let html = self.render_bytes().unwrap_or(Bytes::from(""));
        Ok(html)
      },
      Some(CacheConfig {
        dev_ttl: 1,
        prod_ttl: 3600,
      }),
    ).await {
      Ok(html) => html,
      Err(e) => {
        tracing::error!("Failed to get calendar widget html: {}", e);
        return String::from("");
      }
    };

    String::from_utf8(html.to_vec()).unwrap_or(String::from(""))
  }
}

#[derive(Template)]
#[template(path = "widgets/recommend-blogs.html")]
pub struct RecommendBlogsWidget {
  pub id: String,
  pub class: String,
  pub title: String,
  pub nodes: Vec<NodeDetailModel>,
}

impl Default for RecommendBlogsWidget {
  fn default() -> Self {
    Self {
      id: "side-bar-recommend-blogs".to_string(),
      class: "side-bar-widget".to_string(),
      title: "推荐文章".to_string(),
      nodes: vec![],
    }
  }
}

impl RecommendBlogsWidget {
  pub async fn init(ctx: &WebAppContext) -> Self {
    let mut widget = Self::default();

    // 调用 API 获取节点列表
    let params = NodeFilters {
      page: Some(1),
      page_size: Some(10),
      order_by: Some(String::from("viewed")),
      order: Some(String::from("desc")),
      ..Default::default()
    };
    
    if let Ok(json_res) = describe_node_list(ctx, params).await {
      match json_res.data {
        JsonResPayload::ListData { data, .. } => {
          widget.nodes = data;
        }
        _ => {
          tracing::error!("Failed to get nodes from API response");
        }
      };
    }

    widget
  }

  pub async fn get_html(&self, ctx: &WebAppContext) -> String {
    let html = match get_cached_or_render(
      &ctx.cache,
      format!("widget:{}", self.id).as_str(),
      || async {
        let html = self.render_bytes().unwrap_or(Bytes::from(""));
        Ok(html)
      },
      Some(CacheConfig {
        dev_ttl: 1,
        prod_ttl: 3600,
      }),
    ).await {
      Ok(html) => html,
      Err(e) => {
        tracing::error!("Failed to get calendar widget html: {}", e);
        return String::from("");
      }
    };

    String::from_utf8(html.to_vec()).unwrap_or(String::from(""))
  }
}

// ===== recommend tags =====

#[derive(Template)]
#[template(path = "widgets/recommend-tags.html")]
pub struct RecommendTagsWidget {
  pub id: String,
  pub class: String,
  pub title: String,
  pub tags: Vec<TagModel>,
}

impl Default for RecommendTagsWidget {
  fn default() -> Self {
    Self {
      id: "side-bar-recommend-tags".to_string(),
      class: "side-bar-widget".to_string(),
      title: "热门标签".to_string(),
      tags: vec![],
    }
  }
}

impl RecommendTagsWidget {
  pub async fn init(ctx: &WebAppContext) -> Self {
    let mut widget = Self::default();

    // 调用 API 获取节点列表
    let params = TagFilters {
      page: Some(1),
      page_size: Some(20),
      order_by: Some(String::from("tag_count")),
      order: Some(String::from("desc")),
      ..Default::default()
    };
    
      if let Ok(json_res) = describe_tag_list(ctx, &params).await {
      match json_res.data {
        JsonResPayload::ListData { data, .. } => {
          widget.tags = data;
        }
        _ => {
          tracing::error!("Failed to get nodes from API response");
        }
      };
    }

    widget
  }

  pub async fn get_html(&self, ctx: &WebAppContext) -> String {
    let html = match get_cached_or_render(
      &ctx.cache,
      format!("widget:{}", self.id).as_str(),
      || async {
        let html = self.render_bytes().unwrap_or(Bytes::from(""));
        Ok(html)
      },
      Some(CacheConfig {
        dev_ttl: 1,
        prod_ttl: 3600,
      }),
    ).await {
      Ok(html) => html,
      Err(e) => {
        tracing::error!("Failed to get calendar widget html: {}", e);
        return String::from("");
      }
    };

    String::from_utf8(html.to_vec()).unwrap_or(String::from(""))
  }
}


#[derive(Template)]
#[template(path = "widgets/side-nav.html")]
pub struct SideNavWidget {
  pub key: String,
  pub active_vid: String,
  pub categories: Vec<SideNavItem>,
}

impl SideNavWidget {
  pub async fn get_html(&self, ctx: &WebAppContext) -> String {
    let html = match get_cached_or_render(
      &ctx.cache,
      format!("widget:side-nav:{}:{}",
        self.key.as_str(),
        self.active_vid.as_str()
      ).as_str(),
      || async {
        let html = self.render_bytes().unwrap_or(Bytes::from(""));
        Ok(html)
      },
      Some(CacheConfig {
        dev_ttl: 1,
        prod_ttl: 3600,
      }),
    ).await {
      Ok(html) => html,
      Err(e) => {
        tracing::error!("Failed to get side-nav widget html: {}", e);
        return String::from("");
      }
    };

    String::from_utf8(html.to_vec()).unwrap_or(String::from(""))
  }
}
