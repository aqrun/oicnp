use std::fmt::Debug;
use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use loco_rs::prelude::ModelResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
/// 查 数据返回
pub struct ListData<T> {
    pub data: Vec<T>,
    /// 全部数据条数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 当前分页大小
    #[serde(rename(serialize = "pageSize"))]
    pub page_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum JsonResPayload<T> {
    Data(T),
    ListData {
        data: T,
        pagination: Pagination,
    },
    Empty,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    /// 全部数据条数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 当前分页大小
    #[serde(rename(serialize = "pageSize"))]
    pub page_size: u64,
}

/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct PageParams {
    pub page: Option<u64>,
    #[serde(rename(serialize = "pageSize"))]
    pub page_size: Option<u64>,
}

/// 填入到extensions中的数据
#[derive(Debug, Clone)]
pub struct ResJsonString(pub String);

///
/// 接口JSON数据统一返回格式
///
/// ## 正常成功返回类型
///
/// 调用方式：
///
/// ```rust
/// let user = User {
///     uid: 1,
///     username: String::from("alex"),
/// };
///
/// JsonRes::ok(user)
/// ```
///
/// 接口返回:
///
/// ```json
/// {
///     "code": "200",
///     "data": {
///         "uid": 1,
///         "username": "alex"
///     },
///     "message": "success"
/// }
/// ```
///
/// ## 接口错误时，如401
///
/// 调用方式：
///
/// ```rust
/// let err = Err("请重新登录");
///
/// JsonRes::code("401", err)
/// ```
///
/// 接口返回:
///
///
///  ```json
/// {
///     "code": "401",
///     "data": null,
///     "message": "请重新登录"
/// }
/// ```
///
/// ## 指定数据Key时
///
/// 主要有两种接口类型
///
/// * 获取单个数据
/// * 获取列表数据
///
/// ### 获取单个用户详细信息 `key: user`
///
/// 调用方式：
///
/// ```rust
/// let user = User {
///     uid: 1,
///     username: String::from("alex"),
/// };
///
/// JsonRes::from((user, "user"))
/// ```
///
/// 接口返回：
///
/// ```json
/// {
///     "code": "200",
///     "data": {
///         "user": {
///             "uid": 1,
///             "username": "alex",
///             ...
///         }
///     },
///     "message": "success"
/// }
/// ```
///
/// ### 获取用户列表信息 `key: users`
///
/// /// ```rust
/// let users = vec![
///     User {
///         uid: 1,
///         username: String::from("alex"),
///     },
///     User {
///         uid: 2,
///         username: String::from("bob"),
///     },
/// ];
/// let pager = Pagination {
///     total: 100,
///     page: 1,
///     page_size: 10,
/// }
///
///
/// JsonRes::from(user, pager, "users")
/// ```
///
/// ```json
/// {
///     "code": "200",
///     "data": {
///         "users": [
///             { "uid": 1, "username": "alex", },
///             { "uid": 2, "username": "bob", }
///         ],
///         "total": 100,
///         "page": 1,
///         "pageSize": 10
///     },
///     "message": "success"
/// }
/// ```
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRes<T> {
    /// 返回结果数据Key
    pub wrap_key: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
    pub data: JsonResPayload<T>,
}

/// IntoResponse trait
#[allow(unconditional_recursion)]
impl<T> IntoResponse for JsonRes<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let data = self.to_json();
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

        // 方便后序数据处理
        response.extensions_mut().insert(res_json_string);
        response
    }
}

impl<T> JsonRes<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    pub fn ok(data: T) -> Self {
        Self {
            code: Some(String::from("200")),
            data: JsonResPayload::Data(data),
            message: Some("success".to_string()),
            wrap_key: None,
        }
    }

    pub fn from_str(err: &str) -> Self {
        Self {
            code: Some(String::from("400")),
            data: JsonResPayload::Empty,
            message: Some(err.to_string()),
            wrap_key: None,
        }
    }
    pub fn err(err: impl ToString) -> Self {
        Self {
            code: Some(String::from("400")),
            data: JsonResPayload::Empty,
            message: Some(err.to_string()),
            wrap_key: None,
        }
    }
    pub fn code(code: &str, msg: &str) -> Self {
        Self {
            code: Some(String::from(code)),
            data: JsonResPayload::Empty,
            message: Some(msg.to_string()),
            wrap_key: None,
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

    /// 指定返回结果数据Key
    pub fn wrap(&mut self, wrap_key: &str) -> &mut Self {
        self.wrap_key = Some(String::from(wrap_key));
        self
    }

    /// 直接处理 ModelResult 类型
    pub fn from_result_data(res: ModelResult<T>) -> Self {
        match res {
            Ok(data) => {
                Self {
                    code: Some(String::from("200")),
                    data: JsonResPayload::Data(data),
                    message: Some("success".to_string()),
                    wrap_key: None,
                }
            },
            Err(err) => Self::err(err),
        }
    }

    /// 指定一般详细数据 和 结果key
    pub fn wrap_data(data: T, wrap_key: &str) -> Self {
        Self {
            code: Some(String::from("200")),
            data: JsonResPayload::Data(data),
            message: Some("success".to_string()),
            wrap_key: Some(String::from(wrap_key)),
        }
    }

    /// 指定列表数据 和 结果key
    pub fn wrap_list_data(list_data: JsonResPayload<T>, wrap_key: &str) -> Self {
        Self {
            code: Some(String::from("200")),
            data: list_data,
            message: Some("success".to_string()),
            wrap_key: Some(String::from(wrap_key)),
        }
    }

    ///
    /// 根据数据类型和key返回标准JSON格式
    ///
    pub fn to_json(&self) -> serde_json::Value {
        let mut wrap_key = String::from("");

        if let Some(x) = &self.wrap_key {
            wrap_key = String::from(x);
        }

        match &self.data {
            JsonResPayload::Data(data) => {
                if !wrap_key.is_empty() {
                    serde_json::json!({
                        "code": self.code.as_ref().unwrap_or(&"200".to_string()),
                        "data": {
                            wrap_key: data,
                        },
                        "message": self.message.as_ref().unwrap_or(&"success".to_string()),
                    })
                } else {
                    serde_json::json!({
                        "code": self.code.as_ref().unwrap_or(&"200".to_string()),
                        "data": data,
                        "message": self.message.as_ref().unwrap_or(&"success".to_string()),
                    })
                }
            },
            JsonResPayload::ListData {
                data,
                pagination,
            } => {
                let valid_key = if wrap_key.is_empty() {
                    String::from("list")
                } else {
                    wrap_key
                };
                
                serde_json::json!({
                    "code": self.code.as_ref().unwrap_or(&"200".to_string()),
                    "data": {
                        valid_key: data,
                        "page": pagination.page,
                        "pageSize": pagination.page_size,
                        "total": pagination.total,
                    },
                    "message": self.message.as_ref().unwrap_or(&"success".to_string()),
                })
            },
            JsonResPayload::Empty => {
                serde_json::json!({
                    "code": self.code.as_ref().unwrap_or(&"200".to_string()),
                    "data": serde_json::Value::Null,
                    "message": self.message.as_ref().unwrap_or(&"success".to_string()),
                })
            }
        }
    }
}

///
/// 普通数据 T 和 wrap_key
///
impl<T> From<(T, &str)> for JsonRes<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn from((data, wrap_key): (T, &str)) -> Self {
        Self {
            code: Some(String::from("200")),
            data: JsonResPayload::Data(data),
            message: Some("success".to_string()),
            wrap_key: Some(wrap_key.to_string()),
        }
    }
}

///
/// 列表数据 T 和 wrap_key
///
impl<T> From<(T, Pagination, &str)> for JsonRes<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn from((list_data, pagination, wrap_key): (T, Pagination, &str)) -> Self {
        Self {
            code: Some(String::from("200")),
            data: JsonResPayload::ListData {
                data: list_data,
                pagination,
            },
            message: Some("success".to_string()),
            wrap_key: Some(wrap_key.to_string()),
        }
    }
}

impl<T> From<ModelResult<T>> for JsonRes<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn from(res: ModelResult<T>) -> Self {
        match res {
            Ok(data) => Self::ok(data),
            Err(err) => Self::err(err),
        }
    }
}

impl<T> From<anyhow::Result<T>> for JsonRes<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn from(res: anyhow::Result<T>) -> Self {
        match res {
            Ok(data) => Self::ok(data),
            Err(err) => Self::err(err),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{json, Value};
    use serde::{Serialize, Deserialize};

    /// 获取JSON数据指定Key
    fn get_data_by_key(val: Value, key: &str) -> Value {
        if let Some(data) = val.get("data") {
            if let Some(data) = data.get(key) {
                return data.clone();
            }
        }

        return Value::Null;
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    #[serde(default)]
    struct User {
        pub id: i64,
        pub name: String,
    }

    ///
    /// {
    ///   "code": "200",
    ///   "data": "test data",
    /// }
    ///
    #[test]
    fn test_json_res_普通字符串数据() {
        let a = JsonRes::ok("test data");
        let json_res = a.to_json();
        let data = json_res.get("data");
        assert_eq!(data, Some(&json!("test data")));
    }

    ///
    /// {
    ///   "code": "200",
    ///   "data": {
    ///         "user": "test data",
    ///     },
    /// }
    ///
    #[test]
    fn test_json_res_指定普通数据和key() {
        let a = JsonRes::from(("test data", "user"));
        let json_res = a.to_json();
        let data = get_data_by_key(json_res, "user");
        assert_eq!(data, json!("test data"));
    }

    /// 指定 ModelResult<T> 类型数据和 key
    ///
    /// {
    ///   "code": "200",
    ///   "data": {
    ///         "user": {
    ///             "id": 1,
    ///             "name": "alex",
    ///          },
    ///     },
    /// }
    ///
    #[test]
    fn test_json_res_model_result_data_and_key() {
        // 接口输出的用户数据
        let user = User {
            id: 1,
            name: String::from("alex"),
        };

        // 转为 JsonRes 类型
        let a = JsonRes::from((user, "user"));
        // 接口JSON结果
        let json_res = a.to_json();
        let data = get_data_by_key(json_res, "user");
        // 根据结果JSON反解析用户数据
        let user_parsed = serde_json::from_value::<User>(data).unwrap();
        assert_eq!(user_parsed.name, String::from("alex"));
    }

    /// 指定 ModelResult<ListData<T>> 类型数据和 key
    ///
    /// {
    ///   "code": "200",
    ///   "data": {
    ///         "users": [
    ///            { id: 1, name: alex },
    ///            { id: 2, name: bob },
    ///         ]
    ///     },
    /// }
    ///
    #[test]
    fn test_json_res_wrap_list_data_and_key() {
        // 接口输出的用户数据
        let user_list = vec![
            User {
                id: 1,
                name: String::from("alex"),
            },
            User {
                id: 2,
                name: String::from("bob"),
            },
        ];

        let pagination  = Pagination {
            total: 10,
            page: 1,
            page_size: 10
        };

        let a = JsonRes::from((user_list, pagination, "users"));

        // 接口JSON结果
        let json_res = a.to_json();
        let user_list_data = get_data_by_key(json_res.clone(), "users");
        // 根据结果JSON反解析用户列表数据
        let user_list_parsed = serde_json::from_value::<Vec<User>>(user_list_data).unwrap();
        assert_eq!(user_list_parsed.len(), 2);
    }
}