use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use oic_core::{
    models::nodes::{NodeFilters, NodeDetailModel},
    typings::JsonRes,
};

/// 获取 API 基础 URL
fn get_api_base_url() -> String {
    std::env::var("API_BASE_URL")
        .unwrap_or_else(|_| "https://api.lxage.com".to_string())
}

/// 创建 HTTP 客户端
fn create_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client")
}

/// 节点列表响应（包含分页信息）
#[derive(Debug, Clone)]
pub struct NodeListResponse {
    pub nodes: Vec<NodeDetailModel>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

/// 调用节点列表 API
pub async fn describe_node_list(
    params: NodeFilters,
) -> Result<NodeListResponse> {
    let client = create_client();
    let url = format!("{}/v1/node/list", get_api_base_url());

    let response = client
        .post(&url)
        .json(&params)
        .send()
        .await?;

    let json_res: JsonRes<Value> = response.json().await?;

    if !json_res.is_success() {
        return Err(anyhow::anyhow!(
            "API error: {}",
            json_res.get_msg()
        ));
    }

    // 解析响应数据
    // API 返回格式: { "code": "200", "data": { "nodes": [...], "page": 1, "pageSize": 10, "total": 100 }, "message": "success" }
    let json_value = json_res.to_json();
    let data = json_value.get("data")
        .ok_or_else(|| anyhow::anyhow!("Missing 'data' field in API response"))?;

    let nodes: Vec<NodeDetailModel> = data.get("nodes")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    let total = data.get("total")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let page = data.get("page")
        .and_then(|v| v.as_u64())
        .unwrap_or(1);

    let page_size = data.get("pageSize")
        .and_then(|v| v.as_u64())
        .unwrap_or(10);

    Ok(NodeListResponse {
        nodes,
        total,
        page,
        page_size,
    })
}

/// 调用节点详情 API
pub async fn describe_node_detail(
    params: NodeFilters,
) -> Result<Option<NodeDetailModel>> {
    let client = create_client();
    let url = format!("{}/v1/node/one", get_api_base_url());

    let response = client
        .post(&url)
        .json(&params)
        .send()
        .await?;

    let json_res: JsonRes<Value> = response.json().await?;

    if !json_res.is_success() {
        return Err(anyhow::anyhow!(
            "API error: {}",
            json_res.get_msg()
        ));
    }

    // 解析响应数据
    // API 返回格式: { "code": "200", "data": { "node": {...} }, "message": "success" }
    let json_value = json_res.to_json();
    let data = json_value.get("data")
        .ok_or_else(|| anyhow::anyhow!("Missing 'data' field in API response"))?;

    let node: Option<NodeDetailModel> = data.get("node")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    Ok(node)
}

