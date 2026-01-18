use askama::Template;
use crate::models::{AssetFiles, RenderBytes};
use crate::services::describe_node_list;
use crate::WebAppContext;
use oic_core::{
    models::nodes::{NodeFilters, NodeDetailModel},
    typings::JsonResPayload,
};
use anyhow::Result;
use bytes::Bytes;
use super::{CalendarWidget, RecommendBlogsWidget, RecommendTagsWidget};

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub big_news: Option<NodeDetailModel>,
    pub big_news_items: Vec<NodeDetailModel>,
    pub article_items: Vec<NodeDetailModel>,
    pub assets: AssetFiles,
    pub side_widgets: Vec<String>,
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

pub async fn render_home_index(ctx: &WebAppContext) -> Result<Bytes> {
    // 调用 API 获取节点列表
    let node_filters = NodeFilters {
        page: Some(1),
        page_size: Some(11),
        ..Default::default()
    };
    
    let json_res = describe_node_list(ctx, node_filters).await?;
    
    // 从 JsonRes 中提取节点列表
    let nodes = match json_res.data {
        JsonResPayload::ListData { data, .. } => data,
        _ => {
            tracing::error!("HomePage]Failed to get nodes from API response");
            vec![]
        }
    };
    
    // 第一个作为大新闻
    let big_news = nodes.first().cloned();

    // 接下来的 4 个作为新闻网格
    let big_news_items = nodes.clone().into_iter()
        .skip(1)
        .take(4)
        .collect();
    
    // 剩余的文章列表
    let article_items = nodes.into_iter()
        .skip(5)
        .collect();
    
    let assets = AssetFiles::default();
    
    let template = HomeTemplate {
        ctx: ctx.clone(),
        menu_vid: String::from("home"),
        big_news,
        big_news_items,
        article_items,
        assets,
        side_widgets: vec![
            CalendarWidget::default().get_html(ctx).await,
            RecommendBlogsWidget::init(ctx).await.get_html(ctx).await,
            RecommendTagsWidget::init(ctx).await.get_html(ctx).await,
        ],
    };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}