#![allow(dead_code)]
 
use askama::Template;
use crate::models::{AssetFiles, POETRY_CATEGORIES};
use crate::services::{describe_poetry_list_page_data, describe_poetry_list_with_chapters};
use oic_core::{
    models::poetry::{PoetryFilters, PoetryListDataModel},
    entities::poetry::ChapterModel,
    middleware::HtmxRequest,
};
use crate::WebAppContext;
use anyhow::Result;
use bytes::Bytes;
use oic_html::minify_html;
use super::{CalendarWidget, RecommendBlogsWidget, RecommendTagsWidget, SideNavWidget};
use crate::models::poetry::PoetryListParams;

/// 安全地按字符数截取字符串（不添加省略号）
fn truncate_chars(s: &str, max_chars: usize) -> (String, bool) {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() > max_chars {
        (chars[..max_chars].iter().collect(), true)
    } else {
        (s.to_string(), false)
    }
}

#[derive(Template)]
#[template(path = "poetry/index.html")]
pub struct PoetryHomeTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub poetry_by_category: Vec<PoetryCategoryGroup>,
    pub assets: AssetFiles,
    pub side_nav: String,
    pub side_widgets: Vec<String>,
    pub has_sidebar_left: bool,
}

#[derive(Clone, Debug)]
pub struct PoetryCategoryGroup {
    pub category_id: String,
    pub category_name: String,
    pub poetry_list: Vec<PoetryItemWithChapters>,
    pub more_link: String,
}

#[derive(Clone, Debug)]
pub struct PoetryItemWithChapters {
    pub poetry: PoetryListDataModel,
    pub chapters: Vec<ChapterModel>,
    pub author_name: String,
    pub title_truncated: String,
    pub title_needs_tooltip: bool,
}

#[derive(Template)]
#[template(path = "poetry/list.html")]
pub struct PoetryListTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub poetry_list: Vec<PoetryItemWithChapters>,
    pub category_title: String,
    pub page: u64,
    pub total: u64,
    pub page_size: u64,
    pub has_more: bool,
    pub more_uri: String,
    pub assets: AssetFiles,
    pub side_nav: String,
    pub side_widgets: Vec<String>,
    pub has_sidebar_left: bool,
    pub is_htmx: bool,
}

#[derive(Template)]
#[template(path = "poetry/poetry-list.html")]
pub struct PoetryListPartialTemplate {
    pub poetry_list: Vec<PoetryItemWithChapters>,
    pub page: u64,
    pub total: u64,
    pub page_size: u64,
    pub has_more: bool,
    pub more_uri: String,
    pub category_title: String,
}

#[derive(Template)]
#[template(path = "poetry/detail.html")]
pub struct PoetryDetailTemplate {
    pub ctx: WebAppContext,
    pub menu_vid: String,
    pub poetry: PoetryListDataModel,
    pub chapters: Vec<ChapterModel>,
    pub assets: AssetFiles,
    pub side_widgets: Vec<String>,
    pub has_sidebar_left: bool,
    pub is_book: bool,
    pub author_name: String,
    pub has_wenyanwen: bool,
}

/// 诗词首页：按分类分组显示
pub async fn render_poetry_home(
    ctx: &WebAppContext,
) -> Result<Bytes> {
    // 获取所有分类的 tags
    let filter_tags: Vec<String> = POETRY_CATEGORIES.iter()
        .filter_map(|cat| cat.tags.as_ref())
        .flatten()
        .cloned()
        .collect();
    
    let params = PoetryFilters {
        tags: Some(filter_tags.join(",")),
        poetry_amount: Some(6),
        chapter_amount: Some(5),
        ..Default::default()
    };
    
    tracing::info!("获取诗词列表首页数据 list-page-data: {:?}", params.clone());
    let response = describe_poetry_list_page_data(ctx, &params).await?;
    
    // 按分类分组
    let mut poetry_by_category: Vec<PoetryCategoryGroup> = Vec::new();
    
    for category in POETRY_CATEGORIES.iter() {
        if category.vid == "all" {
            continue;
        }
        
        // 筛选属于该分类的诗词
        let category_poetry: Vec<PoetryListDataModel> = response.poetry_list.iter()
            .filter(|poetry| {
                if let Some(cat_tags) = &category.tags {
                    cat_tags.iter().any(|tag| poetry.tags.contains(tag))
                } else {
                    false
                }
            })
            .take(6)
            .cloned()
            .collect();
        
        if !category_poetry.is_empty() {
            // 为每个诗词准备章节列表
            let poetry_with_chapters: Vec<PoetryItemWithChapters> = category_poetry.iter()
                .map(|poetry| {
                    let chapters: Vec<ChapterModel> = response.chapter_list.iter()
                        .filter(|ch| ch.poetry_id == poetry.id)
                        .take(5)
                        .cloned()
                        .collect();
                    let (title_truncated, title_needs_tooltip) = truncate_chars(&poetry.title, 10);
                    PoetryItemWithChapters {
                        poetry: poetry.clone(),
                        chapters,
                        author_name: poetry.author_name.clone().unwrap_or_default(),
                        title_truncated,
                        title_needs_tooltip,
                    }
                })
                .collect();
            
            poetry_by_category.push(PoetryCategoryGroup {
                category_id: category.vid.clone(),
                category_name: category.name.clone(),
                poetry_list: poetry_with_chapters,
                more_link: format!("/poetry/cat/{}", category.vid),
            });
        }
    }
    
    let side_nav = SideNavWidget {
        key: "poetry".to_string(),
        active_vid: String::from("all"),
        categories: POETRY_CATEGORIES.clone(),
    }.get_html(ctx).await;
    
    let side_widgets = vec![
        CalendarWidget::default().get_html(ctx).await,
        RecommendBlogsWidget::init(ctx).await.get_html(ctx).await,
        RecommendTagsWidget::init(ctx).await.get_html(ctx).await,
    ];
    let assets = AssetFiles::default();
    let template = PoetryHomeTemplate {
        ctx: ctx.clone(),
        menu_vid: String::from("poetry"),
        poetry_by_category,
        assets,
        side_nav,
        side_widgets,
        has_sidebar_left: true,
    };
    
    let html = template.render().unwrap_or_default();
    let html = minify_html(&html);
    Ok(Bytes::from(html))
}

/// 分类诗词列表页
pub async fn render_poetry_list(
    ctx: &WebAppContext,
    params: &PoetryListParams,
    htmx: &HtmxRequest,
) -> Result<Bytes> {
    let mut active_vid = String::from("all");
    let mut page = 1;
    let page_size = params.page_size.unwrap_or(10);
    
    if let Some(x) = &params.cat_vid {
        active_vid = String::from(x);
    }
    if let Some(x) = params.page {
        page = x;
    }
    
    // 查找分类信息
    let category = POETRY_CATEGORIES.iter()
        .find(|c| c.vid == active_vid);
    
    // 构建 API 参数
    let mut api_params = PoetryFilters {
        page: Some(page),
        page_size: Some(page_size),
        chapter_amount: Some(5),
        order: Some("asc".to_string()),
        order_by: Some("id".to_string()),
        ..Default::default()
    };
    
    if let Some(cat) = category {
        if let Some(tags) = &cat.tags {
            api_params.tags = Some(tags.join(","));
        }
        if let Some(dynasty) = &cat.dynasty {
            api_params.dynasty = Some(dynasty.clone());
        }
    }
    
    tracing::info!("获取诗词列表首页数据 list-page-data: {:?}", api_params.clone());
    let response = describe_poetry_list_with_chapters(ctx, &api_params).await?;
    
    // 为每个诗词准备章节列表
    let poetry_with_chapters: Vec<PoetryItemWithChapters> = response.poetry_list.iter()
        .map(|poetry| {
            let chapters: Vec<ChapterModel> = response.chapter_list.iter()
                .filter(|ch| ch.poetry_id == poetry.id)
                .cloned()
                .collect();
            let (title_truncated, title_needs_tooltip) = truncate_chars(&poetry.title, 10);
            PoetryItemWithChapters {
                poetry: poetry.clone(),
                chapters,
                author_name: poetry.author_name.clone().unwrap_or_default(),
                title_truncated,
                title_needs_tooltip,
            }
        })
        .collect();
    
    let has_more = (response.page * response.page_size) < response.total;
    let next_page = response.page + 1;
    let more_uri = format!("/poetry/cat/{}", active_vid);
    
    let category_title = category.map(|c| c.name.clone()).unwrap_or_default();
    
    // HTMX 请求直接返回部分内容
    if htmx.is_htmx {
        let template = PoetryListPartialTemplate {
            poetry_list: poetry_with_chapters,
            page: next_page,
            total: response.total,
            page_size: response.page_size,
            has_more,
            more_uri,
            category_title: String::from(category_title.as_str()),
        };
        let html = template.render().unwrap_or_default();
        let html = minify_html(&html);
        return Ok(Bytes::from(html));
    }
    
    let side_nav = SideNavWidget {
        key: "poetry".to_string(),
        active_vid,
        categories: POETRY_CATEGORIES.clone(),
    }.get_html(ctx).await;
    
    let side_widgets = vec![
        CalendarWidget::default().get_html(ctx).await,
        RecommendBlogsWidget::init(ctx).await.get_html(ctx).await,
        RecommendTagsWidget::init(ctx).await.get_html(ctx).await,
    ];
    let assets = AssetFiles::default();
    let template = PoetryListTemplate {
        ctx: ctx.clone(),
        menu_vid: String::from("poetry"),
        poetry_list: poetry_with_chapters,
        category_title,
        page: next_page,
        total: response.total,
        page_size: response.page_size,
        has_more,
        more_uri,
        assets,
        side_nav,
        side_widgets,
        has_sidebar_left: true,
        is_htmx: htmx.is_htmx,
    };
    
    let html = template.render().unwrap_or_default();
    let html = minify_html(&html);
    Ok(Bytes::from(html))
}

/// 诗词详情页
pub async fn render_poetry_detail(
    ctx: &WebAppContext,
    uuid: String,
) -> Result<Bytes> {
    let params = PoetryFilters {
        uuid: Some(uuid),
        chapter_amount: Some(100),
        page: Some(1),
        page_size: Some(1),
        order: Some("asc".to_string()),
        order_by: Some("id".to_string()),
        ..Default::default()
    };
    
    tracing::info!("获取诗词详情数据 list-with-chapters: {:?}", params.clone());
    let response = describe_poetry_list_with_chapters(ctx, &params).await?;
    
    let poetry = response.poetry_list.first()
        .cloned()
        .unwrap_or_default();
    
    let chapters: Vec<ChapterModel> = response.chapter_list.iter()
        .filter(|ch| ch.poetry_id == poetry.id)
        .cloned()
        .collect();
    
    let is_book = poetry.is_book.as_ref().map(|s| s == "1").unwrap_or(false);
    let author_name = poetry.author_name.clone().unwrap_or_default();
    let has_wenyanwen = poetry.tags.contains("文言文");
    
    let assets = AssetFiles::default();
    let template = PoetryDetailTemplate {
        ctx: ctx.clone(),
        menu_vid: String::from("poetry"),
        poetry,
        chapters,
        assets,
        side_widgets: vec![],
        has_sidebar_left: false,
        is_book,
        author_name,
        has_wenyanwen,
    };
    
    let html = template.render().unwrap_or_default();
    let html = minify_html(&html);
    Ok(Bytes::from(html))
}
