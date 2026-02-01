use anyhow::Result;
use oic_core::{
    models::{
        nodes::{NodeFilters, NodeDetailModel},
        tags::TagFilters,
        poetry::{PoetryFilters, PoetryListPageDataResponse},
    },
    entities::{prelude::*, poetry::ChapterModel},
};
use oic_core::typings::JsonRes;
use super::{call_api, parse_list_response, parse_single_response};
use crate::WebAppContext;
use serde_json::Value;

/// 调用节点列表 API，返回 JsonRes<Vec<NodeDetailModel>>
pub async fn describe_node_list(
    ctx: &WebAppContext,
    params: NodeFilters,
) -> Result<JsonRes<Vec<NodeDetailModel>>> {
    let url = format!("{}/v1/node/list", ctx.config.api_url);
    let json_value = call_api::<NodeFilters>(&url, &params).await?;
    parse_list_response(json_value, "nodes")
}

/// 调用节点详情 API，返回 JsonRes<NodeDetailModel>
pub async fn describe_node_detail(
    ctx: &WebAppContext,
    params: NodeFilters,
) -> Result<JsonRes<NodeDetailModel>> {
    let url = format!("{}/v1/node/one", ctx.config.api_url);
    let json_value = call_api::<NodeFilters>(&url, &params).await?;
    parse_single_response(json_value, "node")
}

pub async fn describe_tag_list(
    ctx: &WebAppContext,
    params: &TagFilters,
) -> Result<JsonRes<Vec<TagModel>>> {
    let url = format!("{}/v1/tag/list", ctx.config.api_url);
    let json_value = call_api::<TagFilters>(&url, params).await?;
    parse_list_response(json_value, "tags")
}

/// 解析 poetry API 响应，返回包含 poetry_list 和 chapter_list 的结构
fn parse_poetry_entry_response(json_value: Value) -> Result<PoetryListPageDataResponse> {
    let data_obj = json_value.get("data")
        .ok_or_else(|| anyhow::anyhow!("Missing 'data' field"))?;
    
    let entry_obj = data_obj.get("entry")
        .ok_or_else(|| anyhow::anyhow!("Missing 'entry' field"))?;
    
    let poetry_list: Vec<oic_core::models::poetry::PoetryListDataModel> = entry_obj
        .get("poetry_list")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    
    let chapter_list: Vec<ChapterModel> = entry_obj
        .get("chapter_list")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    
    let total = entry_obj.get("total")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    
    let page = entry_obj.get("page")
        .and_then(|v| v.as_u64())
        .unwrap_or(1);
    
    let page_size = entry_obj.get("pageSize")
        .or_else(|| entry_obj.get("page_size"))
        .and_then(|v| v.as_u64())
        .unwrap_or(10);
    
    Ok(PoetryListPageDataResponse {
        poetry_list,
        chapter_list,
        total,
        page,
        page_size,
    })
}

/// 调用诗词首页数据 API
pub async fn describe_poetry_list_page_data(
    ctx: &WebAppContext,
    params: &PoetryFilters,
) -> Result<PoetryListPageDataResponse> {
    let url = format!("{}/v1/poetry/list-page-data", ctx.config.api_url);
    let json_value = call_api::<PoetryFilters>(&url, params).await?;
    parse_poetry_entry_response(json_value)
}

/// 调用诗词列表（带章节）API
pub async fn describe_poetry_list_with_chapters(
    ctx: &WebAppContext,
    params: &PoetryFilters,
) -> Result<PoetryListPageDataResponse> {
    let url = format!("{}/v1/poetry/list-with-chapters", ctx.config.api_url);
    let json_value = call_api::<PoetryFilters>(&url, params).await?;
    parse_poetry_entry_response(json_value)
}