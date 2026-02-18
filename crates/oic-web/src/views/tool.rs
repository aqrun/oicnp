#![allow(dead_code)]
use askama::Template;
use crate::models::{AssetFiles, RenderBytes, TOOL_CATEGORIES};
use crate::models::tool::{ALL_TOOLS, get_tools_by_category, ToolItem};
use crate::WebAppContext;
use anyhow::Result;
use bytes::Bytes;
use super::{CalendarWidget, RecommendBlogsWidget, RecommendTagsWidget, SideNavWidget};
use crate::models::tool::ToolListParams;

#[derive(Template)]
#[template(path = "tool/index.html")]
pub struct ToolListTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub tools: Vec<ToolItem>,  // 分类页：直接显示工具列表
    pub tools_by_category: Vec<ToolCategoryGroup>,  // 首页：按分类分组
    pub is_home: bool,  // 是否是首页
    pub category_title: String,  // 分类页的标题（空字符串表示无标题）
    pub assets: AssetFiles,
    pub side_nav: String,
    pub side_widgets: Vec<String>,
    pub has_sidebar_left: bool,
}

#[derive(Clone)]
pub struct ToolCategoryGroup {
    pub category_id: String,
    pub category_name: String,
    pub tools: Vec<ToolItem>,
    pub more_link: String,  // 空字符串表示无链接
}

pub async fn render_tool_list(
    ctx: &WebAppContext,
    params: &ToolListParams,
) -> Result<Bytes> {
    let mut active_vid = String::from("all");
    let mut is_category_page = false;

    // 获取分类
    if let Some(x) = &params.cat_vid {
        active_vid = String::from(x);
    }
    if let Some(x) = params.is_category_page {
        is_category_page = x;
    }

    let side_nav = SideNavWidget {
        key: "tool".to_string(),
        active_vid: active_vid.clone(),
        categories: TOOL_CATEGORIES.clone(),
    }.get_html(ctx).await;
    
    let side_widgets = vec![
        CalendarWidget::default().get_html(ctx).await,
        RecommendBlogsWidget::init(ctx).await.get_html(ctx).await,
        RecommendTagsWidget::init(ctx).await.get_html(ctx).await,
    ];
    let assets = AssetFiles::default();

    // 如果是首页，按分类分组显示
    if active_vid == "all" && !is_category_page {
        let mut tools_by_category: Vec<ToolCategoryGroup> = Vec::new();
        
        // 常用推荐（前 6 个工具，不限制分类）
        let recommended_tools = ALL_TOOLS.iter().take(6).cloned().collect();
        tools_by_category.push(ToolCategoryGroup {
            category_id: "recommended".to_string(),
            category_name: "常用推荐".to_string(),
            tools: recommended_tools,
            more_link: String::from(""),  // 无链接
        });

        // 按分类分组（排除"全部"），每个分类只显示前 6 个
        for category in TOOL_CATEGORIES.iter() {
            if category.vid != "all" {
                let mut category_tools = get_tools_by_category(&category.vid);
                if !category_tools.is_empty() {
                    // 只取前 6 个
                    category_tools.truncate(6);
                    tools_by_category.push(ToolCategoryGroup {
                        category_id: category.vid.clone(),
                        category_name: category.name.clone(),
                        tools: category_tools,
                        more_link: format!("/tool/cat/{}", category.vid),
                    });
                }
            }
        }

        let template = ToolListTemplate {
            ctx: ctx.clone(),
            menu_vid: String::from("tool"),
            tools: vec![],
            tools_by_category,
            is_home: true,
            category_title: String::from(""),  // 首页无分类标题
            assets,
            side_nav,
            side_widgets,
            has_sidebar_left: true,
        };
        
        return template.render_bytes();
    }

    // 分类页：显示指定分类的所有工具
    let tools = get_tools_by_category(&active_vid);
    let category_title = TOOL_CATEGORIES.iter()
        .find(|c| c.vid == active_vid)
        .map(|c| c.name.clone())
        .unwrap_or_default();

    let template = ToolListTemplate {
        ctx: ctx.clone(),
        menu_vid: String::from("tool"),
        tools,
        tools_by_category: vec![],
        is_home: false,
        category_title,
        assets,
        side_nav,
        side_widgets,
        has_sidebar_left: true,
    };
    
    template.render_bytes()
}