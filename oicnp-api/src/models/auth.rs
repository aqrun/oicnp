use oicnp_core::models::auth::LoginInfo;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphqlBody {
    pub operationName: Option<String>,
    pub variables: HashMap<String, String>,
    pub query: String,
}