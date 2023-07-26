use bytes::Bytes;
use oicnp_core::{
    G,
    models::auth::{Claims, LoginInfo},
    services::auth::decode_jwt,
};
use poem::{http::StatusCode, Body, Endpoint, Error, FromRequest, Middleware, Request, Result};
use crate::models::auth::{ReqCtx, GraphqlBody};
use crate::utils::{get_request_auth_token, trim_gql_query, check_is_public_query};
use std::collections::HashMap;
use crate::typings::State;
use std::sync::Arc;

/// req上下文注入中间件 同时进行jwt授权验证
pub struct Context;

impl<E: Endpoint> Middleware<E> for Context {
    type Output = ContextEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ContextEndpoint { inner: ep }
    }
}

/// Endpoint for `Tracing` middleware.
pub struct ContextEndpoint<E> {
    inner: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for ContextEndpoint<E> {
    type Output = E::Output;
    // type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let mut state: Option<State> = None;

        {
            if let Some(state_data) = req.extensions().get::<State>() {
                state = Some(state_data.clone());
            }
        }

        // 请求信息ctx注入
        let auth_token = get_request_auth_token(&req);

        let ori_uri_path = req.original_uri().path().to_string();
        let method = req.method().to_string();
        let path = req.original_uri().path().to_string();
        let path_params = req.uri().query().unwrap_or("").to_string();
        let (req_parts, req_body) = req.into_parts();
        let (bytes, body_data) = match get_body_data(req_body).await {
            Err(e) => return Err(e),
            Ok((x, y)) => (x, y),
        };
        let mut gql_operation_name = String::new();
        let mut gql_variables: HashMap<String, String> =  HashMap::new();
        let mut gql_trimmed_query = String::new();
        let mut gql_is_public_query = false;
        let gql_data = serde_json::from_str::<GraphqlBody>(body_data.as_str());

        if let Ok(gql_data) = gql_data {
            gql_operation_name = gql_data.operationName.unwrap_or("".to_string());
            gql_variables = gql_data.variables;
            gql_trimmed_query = trim_gql_query(gql_data.query.as_str());
            gql_is_public_query = check_is_public_query(gql_trimmed_query.as_str());
        }

        // 解析JWT TOKEN
        let login_info= decode_jwt(
            auth_token.as_str(),
            !gql_is_public_query
        ).unwrap();

        let req_ctx = ReqCtx {
            ori_uri: if path_params.is_empty() { ori_uri_path } else { ori_uri_path + "?" + &path_params },
            path,
            path_params,
            method: method.clone(),
            login_info,
            data: body_data.clone(),
            gql_operation_name,
            gql_variables,
            gql_trimmed_query,
            gql_is_public_query,
        };

        // 生成新的 Request对象
        let mut req = Request::from_parts(req_parts, Body::from(bytes));

        if let Some(state) = state {
            // 移除旧数据
            req.extensions_mut().remove::<State>();
            // 生成新的 State
            let new_state = State {
                schema: state.schema.clone(),
                req_ctx: Some(req_ctx.clone()),
            };
            req.extensions_mut().insert(new_state);
        }

        // 开始请求数据
        self.inner.call(req).await
    }
}

/// 获取body数据
async fn get_body_data(body: Body) -> Result<(Bytes, String)> {
    let bytes = match body.into_bytes().await {
        Ok(v) => v,
        Err(e) => return Err(Error::from_string(e.to_string(), StatusCode::BAD_REQUEST)),
    };
    match std::str::from_utf8(&bytes) {
        Ok(x) => {
            let res_data = x.to_string();
            Ok((bytes, res_data))
        }
        Err(_) => Ok((bytes, "该数据无法转输出，可能为blob，binary".to_string())),
    }
}
