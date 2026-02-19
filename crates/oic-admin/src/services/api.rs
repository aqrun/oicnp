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
