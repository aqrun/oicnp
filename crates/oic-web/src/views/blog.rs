use askama::Template;
use crate::models::ViteAssets;
use crate::services::{describe_node_list, describe_node_detail};
use oic_core::models::nodes::{NodeFilters, NodeDetailModel};
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::ManifestChunk;
use crate::models::HtmlTemplate;
use anyhow::Result;

#[derive(Template)]
#[template(path = "blog_list.html")]
pub struct BlogListTemplate {
    pub cat_vid: Option<String>,
    pub nodes: Vec<NodeDetailModel>,
    pub assets: ViteAssets,
}

#[derive(Template)]
#[template(path = "blog_detail.html")]
pub struct BlogDetailTemplate {
    pub node: Option<NodeDetailModel>,
    pub content: String,
    pub assets: ViteAssets,
}

pub async fn render_blog_list(
    cat_vid: Option<String>,
    manifest: Arc<HashMap<String, ManifestChunk>>
) -> Result<HtmlTemplate<BlogListTemplate>> {
    let mut params = NodeFilters::default();
    params.page = Some(1);
    params.page_size = Some(10);
    params.category_vids = cat_vid.clone();
    
    let response = describe_node_list(params).await?;
    let nodes = response.nodes;
    
    let m: HashMap<String, ManifestChunk> = (*manifest).clone();
    let assets: ViteAssets = ManifestChunk::get_assets_by_name(m, "main");
    
    let template = BlogListTemplate {
        cat_vid,
        nodes,
        assets,
    };
    Ok(HtmlTemplate(template))
}

pub async fn render_blog_detail(
    vid: String,
    manifest: Arc<HashMap<String, ManifestChunk>>
) -> Result<HtmlTemplate<BlogDetailTemplate>> {
    let mut params = NodeFilters::default();
    params.vid = Some(vid);
    
    let node = describe_node_detail(params).await?;
    
    // TODO: 解析 Markdown 内容
    let content = node.as_ref()
        .and_then(|n| n.body.as_ref())
        .cloned()
        .unwrap_or_default();
    
    let m: HashMap<String, ManifestChunk> = (*manifest).clone();
    let assets: ViteAssets = ManifestChunk::get_assets_by_name(m, "main");
    
    let template = BlogDetailTemplate {
        node,
        content,
        assets,
    };
    Ok(HtmlTemplate(template))
}

