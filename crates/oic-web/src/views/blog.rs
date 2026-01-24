use askama::Template;
use crate::models::{AssetFiles, RenderBytes, BLOG_CATEGORIES};
use crate::services::{describe_node_list, describe_node_detail};
use oic_core::{
    models::nodes::{NodeFilters, NodeDetailModel},
    typings::{JsonResPayload, Pagination},
    middleware::HtmxRequest,
};
use crate::WebAppContext;
use anyhow::Result;
use bytes::Bytes;
use super::{CalendarWidget, RecommendBlogsWidget, RecommendTagsWidget, SideNavWidget};
use crate::models::blog::BlogListParams;

#[derive(Template)]
#[template(path = "blog/index.html")]
pub struct BlogListTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub node_list_html: String,
    pub assets: AssetFiles,
    pub side_nav: String,
    pub side_widgets: Vec<String>,
}

#[derive(Template)]
#[template(path = "blog/node-list.html")]
pub struct BlogNodeListTemplate {
    pub nodes: Vec<NodeDetailModel>,
    pub page: u64,
    pub cat_vid: String,
    pub more_uri: String,
    pub has_more: bool,
    pub tag_vid: String,
    pub is_tag_page: bool,
    pub is_htmx: bool,
}

#[derive(Template)]
#[template(path = "blog_detail.html")]
pub struct BlogDetailTemplate {
    pub title: Option<String>,
    pub summary: String, // 改为非 Option，始终有值
    pub category_vid: Option<String>,
    pub category_name: Option<String>,
    pub created_at: Option<String>,
    pub content: String,
    pub assets: AssetFiles,
}

pub async fn render_blog_list(
    ctx: &WebAppContext,
    params: &BlogListParams,
    htmx: &HtmxRequest,
) -> Result<Bytes> {
    let mut is_category_page = false;
    let mut is_tag_page = false;
    let mut active_vid = String::from("all");
    let mut active_tag_vid = String::from("all");

    let mut node_filters = NodeFilters {
        page_size: Some(10),
        ..Default::default()
    };

    if let Some(x) = params.page {
        node_filters.page = Some(x);
    }
    if let Some(x) = params.page_size {
        node_filters.page_size = Some(x);
    }
    if let Some(x) = &params.cat_vid {
        active_vid = String::from(x);
        node_filters.category_vids = Some(String::from(x));
    }
    if let Some(x) = &params.tag_vid {
        active_tag_vid = String::from(x);
        node_filters.tag_vids = Some(String::from(x));
    }
    if let Some(x) = params.is_category_page {
        is_category_page = x;
    }
    if let Some(x) = params.is_tag_page {
        is_tag_page = x;
    }

    let json_res = describe_node_list(
        ctx, node_filters
    ).await?;
    
    // 从 JsonRes 中提取节点列表
    let (nodes, pagination) = match json_res.data {
        JsonResPayload::ListData {
            data,
            pagination,
        } => (data, pagination),
        _ => {
            tracing::error!("BlogLoadMore]Failed to get nodes from API response");
            (vec![], Pagination { total: 0, page: 1, page_size: 10 })
        }
    };

    let has_more = (pagination.page * pagination.page_size) < pagination.total;
    let next_page = pagination.page + 1;

    let mut node_list_template = BlogNodeListTemplate {
        nodes,
        page: next_page,
        cat_vid: String::from(""),
        more_uri: String::from("/blog/"),
        has_more,
        tag_vid: String::from(""),
        is_tag_page: false,
        is_htmx: htmx.is_htmx,
    };
    
    // 分类列表
    if is_category_page {
        node_list_template.more_uri = format!("/cat/{}/", active_vid.as_str());
    } else if is_tag_page {
        // 标签列表
        node_list_template.more_uri = format!("/tag/{}/", active_tag_vid.as_str());
        node_list_template.tag_vid = String::from(active_tag_vid.as_str());
        node_list_template.is_tag_page = true;
    } else {
        // 首页
        node_list_template.more_uri = format!("/blog/");

        if active_vid != "all" {
            node_list_template.cat_vid = String::from(active_vid.as_str());
        }
    }

    if htmx.is_htmx {
        return node_list_template.render_bytes();
    }

    let side_nav = SideNavWidget {
        key: "blog".to_string(),
        active_vid,
        categories: BLOG_CATEGORIES.clone(),
    }.get_html(ctx).await;
    let side_widgets = vec![
        CalendarWidget::default().get_html(ctx).await,
        RecommendBlogsWidget::init(ctx).await.get_html(ctx).await,
        RecommendTagsWidget::init(ctx).await.get_html(ctx).await,
    ];
    let assets = AssetFiles::default();
    let template = BlogListTemplate {
        ctx: ctx.clone(),
        menu_vid: String::from("blog"),
        node_list_html: node_list_template.render().unwrap_or(String::from("")),
        assets,
        side_nav,
        side_widgets,
    };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}

pub async fn render_blog_detail(
    ctx: &WebAppContext,
    vid: String,
) -> Result<Bytes> {
    let mut params = NodeFilters::default();
    params.vid = Some(vid);
    
    let json_res = describe_node_detail(ctx, params).await?;
    
    // 从 JsonRes 中提取节点
    let node = match json_res.data {
        JsonResPayload::Data(node) => Some(node),
        _ => None,
    };
    
    // 提取节点字段
    let title = node.as_ref().map(|n| n.title.clone());
    let summary = node.as_ref()
        .map(|n| n.summary.clone())
        .unwrap_or_default(); // 确保 summary 始终有值（即使为空字符串）
    let category_vid = node.as_ref()
        .and_then(|n| n.categories.first())
        .map(|cat| cat.cat_vid.clone());
    let category_name = node.as_ref()
        .and_then(|n| n.categories.first())
        .map(|cat| cat.cat_name.clone());
    let created_at = node.as_ref()
        .map(|n| n.created_at.format(oic_core::constants::DATE_TIME_FORMAT).to_string());
    
    // TODO: 解析 Markdown 内容
    let content = node.as_ref()
        .and_then(|n| n.body.as_ref())
        .cloned()
        .unwrap_or_default();
    
    let assets = AssetFiles::default();
    
    let template = BlogDetailTemplate {
        title,
        summary,
        category_vid,
        category_name,
        created_at,
        content,
        assets,
    };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}

