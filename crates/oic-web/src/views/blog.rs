use askama::Template;
use crate::models::{AssetFiles, RenderBytes, BLOG_CATEGORIES};
use crate::services::{describe_node_list, describe_node_detail};
use oic_core::{
    models::nodes::{NodeFilters, NodeDetailModel},
    typings::JsonResPayload,
};
use crate::WebAppContext;
use anyhow::Result;
use bytes::Bytes;
use super::{CalendarWidget, RecommendBlogsWidget, RecommendTagsWidget, SideNavWidget};

#[derive(Template)]
#[template(path = "blog/index.html")]
pub struct BlogListTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub nodes: Vec<NodeDetailModel>,
    pub assets: AssetFiles,
    pub side_nav: String,
    pub side_widgets: Vec<String>,
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
    cat_vid: Option<String>,
) -> Result<Bytes> {
    let node_filters = NodeFilters {
        page: Some(1),
        page_size: Some(10),
        category_vids: cat_vid.clone(),
        ..Default::default()
    };

    let json_res = describe_node_list(ctx, node_filters).await?;
    
    // 从 JsonRes 中提取节点列表
    let nodes = match json_res.data {
        JsonResPayload::ListData { data, .. } => data,
        _ => {
            tracing::error!("BlogIndex]Failed to get nodes from API response");
            vec![]
        }
    };

    let side_nav = SideNavWidget {
        key: "blog".to_string(),
        active_vid: cat_vid.clone().unwrap_or(String::from("all")),
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
        nodes,
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

