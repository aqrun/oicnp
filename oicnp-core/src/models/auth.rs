use async_graphql::{self, Object};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ReqCtx {
    pub ori_uri: String,
    pub path: String,
    pub path_params: String,
    pub method: String,
    pub login_info: LoginInfo,
    pub data: String,
    pub gql_operation_name: String,
    pub gql_variables: HashMap<String, String>,
    pub gql_trimmed_query: String,
    pub gql_is_public_query: bool,
}

///
/// 请求上下文默认初始化
///
pub fn init_default_req_ctx() -> ReqCtx {
    ReqCtx {
        ori_uri: String::from(""),
        path: String::from(""),
        path_params: String::from(""),
        method: String::from(""),
        login_info: LoginInfo {
            token: String::from(""),
            uid: 0,
            role: String::from(""),
            exp: 0,
        },
        data: String::from(""),
        gql_operation_name: String::from(""),
        gql_variables: HashMap::new(),
        gql_trimmed_query: String::from(""),
        gql_is_public_query: false,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphqlBody {
    pub operationName: Option<String>,
    pub variables: HashMap<String, String>,
    pub query: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub uid: i64,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginInfo {
    pub token: String,
    pub uid: i64,
    pub role: String,
    pub exp: usize,
}

#[Object]
impl LoginInfo {
    async fn token(&self) -> &str {
        self.token.as_str()
    }
    async fn uid(&self) -> i64 {
        self.uid
    }
    async fn role(&self) -> &str {
        self.role.as_str()
    }
    async fn exp(&self) -> usize {
        self.exp
    }
}
