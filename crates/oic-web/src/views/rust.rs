use askama::Template;
use crate::models::AssetFiles;
use crate::services::describe_node_list;
use oic_core::{
    models::nodes::NodeFilters,
    typings::{JsonResPayload, Pagination},
};
use crate::WebAppContext;
use anyhow::Result;
use bytes::Bytes;
use oic_html::minify_html;
use super::{
    CalendarWidget,
    RecommendBlogsWidget,
    RecommendTagsWidget,
    BlogNodeListTemplate,
};

#[derive(Template)]
#[template(path = "rust/index.html")]
pub struct RustHomeTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub node_list_html: String,
    pub assets: AssetFiles,
    pub side_widgets: Vec<String>,
    pub has_sidebar_left: bool,
}

pub async fn render_rust_home(
    ctx: &WebAppContext,
) -> Result<Bytes> {
    let node_filters = NodeFilters {
        page: Some(1),
        page_size: Some(10),
        tag_vids: Some(String::from("rust")),
        ..Default::default()
    };

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

    let node_list_template = BlogNodeListTemplate {
        nodes,
        page: next_page,
        cat_vid: String::from(""),
        more_uri: String::from("/tag/rust"),
        has_more,
        tag_vid: String::from(""),
        is_tag_page: false,
        is_htmx: false,
    };

    let side_widgets = vec![
        CalendarWidget::default().get_html(ctx).await,
        RecommendBlogsWidget::init(ctx).await.get_html(ctx).await,
        RecommendTagsWidget::init(ctx).await.get_html(ctx).await,
    ];
    let assets = AssetFiles::default();
    let template = RustHomeTemplate {
        ctx: ctx.clone(),
        menu_vid: String::from("rust"),
        node_list_html: node_list_template.render().unwrap_or(String::from("")),
        assets,
        side_widgets,
        has_sidebar_left: false,
    };
    
    let html = template.render().unwrap_or_default();
    let html = minify_html(&html);
    Ok(Bytes::from(html))
}
