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
#[template(path = "tool/index.html")]
pub struct ToolListTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub node_list_html: String,
    pub assets: AssetFiles,
    pub side_nav: String,
    pub side_widgets: Vec<String>,
    pub has_sidebar_left: bool,
}

pub async fn render_tool_list(
    ctx: &WebAppContext,
    params: &BlogListParams,
    htmx: &HtmxRequest,
) -> Result<Bytes> {
    let mut is_category_page = false;
    let mut is_tag_page = false;
    let mut active_vid = String::from("all");
    let mut active_tag_vid = String::from("all");


    let side_nav = SideNavWidget {
        key: "tool".to_string(),
        active_vid,
        categories: BLOG_CATEGORIES.clone(),
    }.get_html(ctx).await;
    let side_widgets = vec![
        CalendarWidget::default().get_html(ctx).await,
        RecommendBlogsWidget::init(ctx).await.get_html(ctx).await,
        RecommendTagsWidget::init(ctx).await.get_html(ctx).await,
    ];
    let assets = AssetFiles::default();
    let template = ToolListTemplate {
        ctx: ctx.clone(),
        menu_vid: String::from("tool"),
        node_list_html: String::from(""),
        assets,
        side_nav,
        side_widgets,
        has_sidebar_left: true,
    };
    
    // 使用 RenderBytes trait 直接渲染为 Bytes
    template.render_bytes()
}

