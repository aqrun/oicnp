use askama::Template;
use crate::models::{AssetFiles, RenderBytes};
use crate::services::{describe_node_list, describe_node_detail};
use oic_core::models::nodes::NodeFilters;
use anyhow::Result;
use bytes::Bytes;

#[derive(Clone)]
pub struct BlogListItem {
    pub vid: String,
    pub title: String,
    pub summary: String,
    pub created_at: String,
    pub category_name: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Template)]
#[template(path = "blog_list.html")]
pub struct BlogListTemplate {
    #[allow(dead_code)]
    pub cat_vid: Option<String>,
    pub nodes: Vec<BlogListItem>,
    pub assets: AssetFiles,
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
    cat_vid: Option<String>,
) -> Result<Bytes> {
    let mut params = NodeFilters::default();
    params.page = Some(1);
    params.page_size = Some(10);
    params.category_vids = cat_vid.clone();
    
    let response = describe_node_list(params).await?;
    let node_models = response.nodes;
    
    // Convert NodeDetailModel to BlogListItem
    let nodes: Vec<BlogListItem> = node_models.iter()
        .map(|node| {
            let category_name = node.categories.first()
                .map(|cat| cat.cat_name.clone());
            let tags: Vec<String> = node.tags.iter()
                .map(|t| t.tag_name.clone())
                .collect();
            BlogListItem {
                vid: node.vid.clone(),
                title: node.title.clone(),
                summary: node.summary.clone(),
                created_at: node.created_at.format(oic_core::constants::DATE_TIME_FORMAT).to_string(),
                category_name,
                tags,
            }
        })
        .collect();
    
    let assets = AssetFiles::default();
    
    let template = BlogListTemplate {
        cat_vid,
        nodes,
        assets,
    };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}

pub async fn render_blog_detail(
    vid: String,
) -> Result<Bytes> {
    let mut params = NodeFilters::default();
    params.vid = Some(vid);
    
    let node = describe_node_detail(params).await?;
    
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

