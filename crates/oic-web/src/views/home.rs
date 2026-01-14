use askama::Template;
use crate::models::{AssetFiles, RenderBytes};
use crate::services::describe_node_list;
use oic_core::models::nodes::{NodeFilters, NodeDetailModel};
use anyhow::Result;
use bytes::Bytes;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub big_news_category: Option<String>,
    pub big_news_vid: Option<String>,
    pub big_news_title: Option<String>,
    pub big_news_summary: Option<String>,
    pub news_items: Vec<NewsItem>,
    pub article_items: Vec<ArticleItem>,
    pub assets: AssetFiles,
}

#[derive(Clone)]
pub struct NewsItem {
    pub node: NodeDetailModel,
    pub category_name: Option<String>,
    pub category_id: Option<i64>,
    pub vid: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub date_year: Option<String>,
    pub date_month: Option<String>,
    pub date_day: Option<String>,
    pub image_index: usize,
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

pub async fn render_home_index() -> Result<Bytes> {
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
        .enumerate()
        .map(|(idx, node)| {
            let category_name = node.categories.first()
                .map(|cat| cat.cat_name.clone());
            let category_id = node.categories.first()
                .map(|cat| cat.cat_id);
            let created_at_str = node.created_at.format(oic_core::constants::DATE_TIME_FORMAT).to_string();
            // 解析日期部分：格式为 "YYYY-MM-DD HH:MM:SS"
            let date_parts: Vec<&str> = created_at_str.split(' ').next()
                .map(|date_part| date_part.split('-').collect::<Vec<&str>>())
                .unwrap_or_default();
            let (date_year, date_month, date_day) = if date_parts.len() >= 3 {
                (Some(date_parts[0].to_string()), Some(date_parts[1].to_string()), Some(date_parts[2].to_string()))
            } else {
                (None, None, None)
            };
            NewsItem {
                node: node.clone(),
                category_name,
                category_id,
                vid: Some(node.vid.clone()),
                title: Some(node.title.clone()),
                created_at: Some(created_at_str),
                date_year,
                date_month,
                date_day,
                image_index: idx + 2, // 图片索引从 2 开始（big-news2.jpeg, big-news3.jpeg, ...）
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
    
    let assets = AssetFiles::default();
    
    let template = HomeTemplate {
        big_news_category,
        big_news_vid,
        big_news_title,
        big_news_summary,
        news_items,
        article_items,
        assets,
    };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}