use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListData<T> {
    pub data: Vec<T>,
    /// 第几页
    pub page: u64,
    /// 每页数据量
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64,
    /// 数据总条数
    pub total_count: u64,
}
