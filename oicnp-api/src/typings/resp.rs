use async_graphql::{Object, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
pub struct JsonResponse {
    code: Value,
    message: Value,
    data: Value,
}

impl JsonResponse {
    pub fn success(data: Value) -> Self {
        Self {
            code: json!("200"),
            message: Value::Null,
            data,
        }
    }

    pub fn error(code: Value, message: Value) -> Self {
        Self {
            code,
            message,
            data: Value::Null,
        }
    }
}

///
/// 分页数据信息
///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PagerInfo {
    pub page: i32,
    pub page_size: i32,
    pub total_count: i32,
}

#[Object]
impl PagerInfo {
    async fn page(&self) -> i32 {
        self.page
    }
    async fn page_size(&self) -> i32 {
        self.page_size
    }
    async fn total_count(&self) -> i32 {
        self.total_count
    }
}
