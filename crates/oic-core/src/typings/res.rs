use std::fmt::Debug;
use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use loco_rs::prelude::ModelResult;

/// 分页数据返回
#[derive(Debug, Serialize)]
pub struct ListData<T> {
    /// 数据列表
    pub data: Vec<T>,
    /// 全部数据条数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 当前分页大小
    #[serde(rename(serialize = "pageSize"))]
    pub page_size: u64,
}

/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct PageParams {
    pub page: Option<u64>,
    #[serde(rename(serialize = "pageSize"))]
    pub page_size: Option<u64>,
}

/// 数据统一返回格式
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct JsonRes {
    pub code: Option<String>,
    pub data: Option<DataWrapper>,
    pub message: Option<String>,
}

/// 数据包装器
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DataWrapper {
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

/// 填入到extensions中的数据
#[derive(Debug, Clone)]
pub struct ResJsonString(pub String);

#[allow(unconditional_recursion)]
impl IntoResponse for JsonRes {
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            data: self.data,
            message: self.message,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE, 
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref())
                    )
                    .body(Body::from(e.to_string()))
                    .unwrap();
            }
        };
        let res_json_string = ResJsonString(json_string.clone());

        let mut response = Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::APPLICATION_JSON.as_ref())
            )
            .body(Body::from(json_string))
            .unwrap();

        response.extensions_mut().insert(res_json_string);
        response
    }
}

impl JsonRes {
    pub fn ok<T: Serialize>(data: T) -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert("data".to_string(), serde_json::to_value(data).unwrap());
        Self {
            code: Some(String::from("200")),
            data: Some(DataWrapper { data: map }),
            message: Some("success".to_string()),
        }
    }

    pub fn from_str(err: &str) -> Self {
        Self {
            code: Some(String::from("400")),
            data: None,
            message: Some(err.to_string()),
        }
    }

    pub fn err(err: impl ToString) -> Self {
        Self {
            code: Some(String::from("400")),
            data: None,
            message: Some(err.to_string()),
        }
    }

    pub fn code(code: &str, msg: &str) -> Self {
        Self {
            code: Some(String::from(code)),
            data: None,
            message: Some(msg.to_string()),
        }
    }

    /// 将数据包装在指定字段名中
    pub fn wrap_data<T: Serialize>(data: T, field_name: &str) -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert(field_name.to_string(), serde_json::to_value(data).unwrap());
        Self {
            code: Some(String::from("200")),
            data: Some(DataWrapper { data: map }),
            message: Some("success".to_string()),
        }
    }

    /// 将 ModelResult 数据包装在指定字段名中
    pub fn wrap_model_result<T: Serialize>(result: ModelResult<T>, field_name: &str) -> Self {
        match result {
            Ok(data) => Self::wrap_data(data, field_name),
            Err(err) => Self::err(err),
        }
    }

    /// 将 anyhow::Result 数据包装在指定字段名中
    pub fn wrap_anyhow_result<T: Serialize>(result: anyhow::Result<T>, field_name: &str) -> Self {
        match result {
            Ok(data) => Self::wrap_data(data, field_name),
            Err(err) => Self::err(err),
        }
    }

    /// 包装列表数据
    pub fn wrap_list_data<T: Serialize>(data: Vec<T>, field_name: &str, total: u64, page: u64, page_size: u64) -> Self {
        let mut map = std::collections::HashMap::new();
        map.insert(field_name.to_string(), serde_json::to_value(data).unwrap());
        map.insert("total".to_string(), serde_json::to_value(total).unwrap());
        map.insert("page".to_string(), serde_json::to_value(page).unwrap());
        map.insert("pageSize".to_string(), serde_json::to_value(page_size).unwrap());
        Self {
            code: Some(String::from("200")),
            data: Some(DataWrapper { data: map }),
            message: Some("success".to_string()),
        }
    }

    /// 包装 ModelResult 列表数据
    pub fn wrap_model_list<T: Serialize>(result: ModelResult<ListData<T>>, field_name: &str) -> Self {
        match result {
            Ok(ListData{
                data,
                page,
                page_size,
                total,
            }) => Self::wrap_list_data(data, field_name, total, page, page_size),
            Err(err) => Self::err(err),
        }
    }

    pub fn is_success(&self) -> bool {
        if self.code.is_none() {
            return true;
        }

        if let Some(code) = &self.code {
            if code.eq("200") {
                return true;
            }
        }

        false
    }

    pub fn get_code(&self) -> String {
        if let Some(code) = &self.code {
            return String::from(code);
        }

        String::from("")
    }

    pub fn get_msg(&self) -> String {
        if let Some(msg) = &self.message {
            return String::from(msg);
        }

        String::from("")
    }
}

impl<T: Serialize> From<ModelResult<T>> for JsonRes {
    fn from(res: ModelResult<T>) -> Self {
        match res {
            Ok(res) => Self::ok(res),
            Err(err) => Self::err(err),
        }
    }
}

impl<T: Serialize> From<anyhow::Result<T>> for JsonRes {
    fn from(res: anyhow::Result<T>) -> Self {
        match res {
            Ok(res) => Self::ok(res),
            Err(err) => Self::err(err),
        }
    }
}
