use async_graphql::Object;
use oicnp_core::models::auth::LoginInfo as CoreLoginInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ReqCtx {
    pub ori_uri: String,
    pub path: String,
    pub path_params: String,
    pub method: String,
    pub login_info: CoreLoginInfo,
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
        login_info: CoreLoginInfo {
            token: String::from(""),
            uid: String::from(""),
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

#[derive(Debug)]
pub struct LoginInfo {
    pub data: CoreLoginInfo,
}

#[Object]
impl LoginInfo {
    async fn token(&self) -> &str {
        self.data.token.as_str()
    }
    async fn uid(&self) -> &str {
        self.data.uid.as_str()
    }
    async fn role(&self) -> &str {
        self.data.role.as_str()
    }
    async fn exp(&self) -> usize {
        self.data.exp
    }
}
