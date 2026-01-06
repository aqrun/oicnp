use askama::Template;
use crate::models::ViteAssets;
use crate::services::{describe_node_list, NodeListResponse};
use oic_core::models::nodes::{NodeFilters, NodeDetailModel};
use std::sync::Arc;
use std::collections::HashMap;
use crate::models::ManifestChunk;
use crate::models::HtmlTemplate;
use anyhow::Result;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub big_news: Option<NodeDetailModel>,
    pub big_news_category: Option<String>,
    pub big_news_vid: Option<String>,
    pub big_news_title: Option<String>,
    pub big_news_summary: Option<String>,
    pub news_items: Vec<NewsItem>,
    pub article_items: Vec<ArticleItem>,
    pub assets: ViteAssets,
}

#[derive(Clone)]
pub struct NewsItem {
    pub node: NodeDetailModel,
    pub category_name: Option<String>,
    pub category_id: Option<i64>,
    pub vid: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Clone)]
pub struct ArticleItem {
    pub node: NodeDetailModel,
    pub category_name: Option<String>,
    pub vid: Option<String>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub created_at: Option<String>,
    pub tags: Vec<String>,
}

pub async fn render_home_index(
    manifest: Arc<HashMap<String, ManifestChunk>>
) -> Result<HtmlTemplate<HomeTemplate>> {
    // 调用 API 获取节点列表
    let mut params = NodeFilters::default();
    params.page = Some(1);
    params.page_size = Some(11);
    
    let response = describe_node_list(params).await?;
    let nodes = response.nodes;
    
    // 第一个作为大新闻
    let big_news = nodes.first().cloned();
    let big_news_category = big_news.as_ref()
        .and_then(|n| n.categories.first())
        .map(|cat| cat.cat_name.clone());
    let big_news_vid = big_news.as_ref().map(|n| n.vid.clone());
    let big_news_title = big_news.as_ref().map(|n| n.title.clone());
    let big_news_summary = big_news.as_ref().map(|n| n.summary.clone());
    
    // 接下来的 4 个作为新闻网格
    let news_items: Vec<NewsItem> = nodes.iter()
        .skip(1)
        .take(4)
        .map(|node| {
            let category_name = node.categories.first()
                .map(|cat| cat.cat_name.clone());
            let category_id = node.categories.first()
                .map(|cat| cat.cat_id);
            NewsItem {
                node: node.clone(),
                category_name,
                category_id,
                vid: Some(node.vid.clone()),
                title: Some(node.title.clone()),
                created_at: Some(node.created_at.format(oic_core::constants::DATE_TIME_FORMAT).to_string()),
            }
        })
        .collect();
    
    // 剩余的文章列表
    let article_items: Vec<ArticleItem> = nodes.iter()
        .skip(5)
        .map(|node| {
            let category_name = node.categories.first()
                .map(|cat| cat.cat_name.clone());
            let tags: Vec<String> = node.tags.iter()
                .map(|t| t.tag_name.clone())
                .collect();
            ArticleItem {
                node: node.clone(),
                category_name,
                vid: Some(node.vid.clone()),
                title: Some(node.title.clone()),
                summary: Some(node.summary.clone()),
                created_at: Some(node.created_at.format(oic_core::constants::DATE_TIME_FORMAT).to_string()),
                tags,
            }
        })
        .collect();
    
    let m: HashMap<String, ManifestChunk> = (*manifest).clone();
    let assets: ViteAssets = ManifestChunk::get_assets_by_name(m, "main");
    
    let template = HomeTemplate {
        big_news,
        big_news_category,
        big_news_vid,
        big_news_title,
        big_news_summary,
        news_items,
        article_items,
        assets,
    };
    Ok(HtmlTemplate(template))
}