use std::fmt::Debug;
use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use loco_rs::prelude::*;

#[derive(Debug, Serialize)]
/// 查 数据返回
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub total_pages: u64,
    pub page_num: u64,
}
/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct PageParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

/// 数据统一返回格式
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct JsonRes<T> {
    pub code: Option<String>,
    pub data: Option<T>,
    pub message: Option<String>,
}

/// 填入到extensions中的数据
#[derive(Debug, Clone)]
pub struct ResJsonString(pub String);

#[allow(unconditional_recursion)]
impl<T> IntoResponse for JsonRes<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
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

impl<T: Serialize> JsonRes<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: Some(String::from("200")),
            data: Some(data),
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
    pub fn is_success(&self) -> bool {
        if self.code.is_none() {
            return true;
        }

        if let Some(code) = &self.code {
            if code.eq("200") {
                return true;
            }
        }

        return false;
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

impl<T> From<ModelResult<T>> for JsonRes<T>
where
    T: Serialize + Send + Sync + Debug,
{
    fn from(res: ModelResult<T>) -> Self {
        match res {
            Ok(res) => Self::ok(res),
            Err(err) => Self::err(err),
        }
    }
}
