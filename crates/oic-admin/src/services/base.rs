use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use oic_core::typings::{JsonRes, JsonResPayload, Pagination};

/// 创建 HTTP 客户端
fn create_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client")
}

/// 通用的 API 调用辅助函数
pub async fn call_api<T>(url: &str, body: &impl Serialize) -> Result<Value>
where
    T: Serialize,
{
    let client = create_client();
    let response = client
        .post(url)
        .json(body)
        .send()
        .await?;
    
    let json_value: Value = response.json().await?;
    
    // 检查错误
    let code = json_value.get("code")
        .and_then(|v| v.as_str())
        .unwrap_or("400");
    
    if code != "200" {
        let message = json_value.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("API error");
        return Err(anyhow::anyhow!("API error [{}]: {}", code, message));
    }
    
    Ok(json_value)
}

/// POST JSON，并附带 `Authorization: Bearer <token>`；成功判定与 `call_api` 一致（`code == "200"`）。
pub async fn call_api_with_bearer(
    url: &str,
    bearer: &str,
    body: &impl Serialize,
) -> Result<Value> {
    let client = create_client();
    let response = client
        .post(url)
        .header(
            "Authorization",
            format!("Bearer {}", bearer.trim()),
        )
        .json(body)
        .send()
        .await?;

    let json_value: Value = response.json().await?;
    tracing::info!("uri: {}, json_value: {:?}", url, json_value);
    Ok(json_value)
}

/// 从 JSON Value 解析列表类型的 JsonRes
pub fn parse_list_response<T>(json_value: Value, data_key: &str) -> Result<JsonRes<Vec<T>>>
where
    T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    let data_obj = json_value.get("data")
        .ok_or_else(|| anyhow::anyhow!("Missing 'data' field"))?;
    
    let list_data: Vec<T> = data_obj.get(data_key)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    
    let pagination = Pagination {
        total: data_obj.get("total").and_then(|v| v.as_u64()).unwrap_or(0),
        page: data_obj.get("page").and_then(|v| v.as_u64()).unwrap_or(1),
        page_size: data_obj.get("pageSize").and_then(|v| v.as_u64()).unwrap_or(10),
    };
    
    Ok(JsonRes {
        wrap_key: Some(data_key.to_string()),
        code: Some("200".to_string()),
        message: json_value.get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        data: JsonResPayload::ListData {
            data: list_data,
            pagination,
        },
    })
}

/// 从 JSON Value 解析单个对象的 JsonRes
pub fn parse_single_response<T>(json_value: Value, data_key: &str) -> Result<JsonRes<T>>
where
    T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    let data_obj = json_value.get("data")
        .ok_or_else(|| anyhow::anyhow!("Missing 'data' field"))?;
    
    let data: Option<T> = data_obj.get(data_key)
        .and_then(|v| serde_json::from_value(v.clone()).ok());
    
    Ok(JsonRes {
        wrap_key: Some(data_key.to_string()),
        code: Some("200".to_string()),
        message: json_value.get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        data: match data {
            Some(item) => JsonResPayload::Data(item),
            None => JsonResPayload::Empty,
        },
    })
}

/// 从 JSON Value 解析单个对象的 JsonRes 不带 wrap_key 的
pub fn parse_response<T>(json_value: Value) -> Result<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    let data: T = serde_json::from_value(json_value)
        .map_err(|e| anyhow::anyhow!("Invalid data field: {}", e))?;

    Ok(data)
}

