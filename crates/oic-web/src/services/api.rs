use anyhow::Result;
use oic_core::{
    models::{
        nodes::{NodeFilters, NodeDetailModel},
        tags::TagFilters,
    },
    entities::prelude::*,
};
use oic_core::typings::JsonRes;
use super::{call_api, parse_list_response, parse_single_response};
use crate::WebAppContext;

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