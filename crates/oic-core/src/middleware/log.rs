use std::{
    convert::Infallible,
    task::{Context, Poll},
    time::Instant,
    sync::Arc,
};

use axum::{
    body::{Body, Bytes},
    extract::{FromRef, FromRequestParts, Request},
    response::Response,
    http::request::Parts,
    Router as AXRouter,
};
use http_body_util::BodyExt;
use futures_util::future::BoxFuture;
use loco_rs::{
    prelude::*,
    controller::middleware::{MiddlewareLayer, request_id::LocoRequestId},
};
use tower::{Layer, Service};
use crate::{
    entities::prelude::*,
    prelude::*,
};
use crate::services::settings::Settings;
use super::{
    JWTWithUser,
    ClientInfo,
};

#[derive(Clone)]
pub struct OperationLogLayer {
    state: AppContext,
}

impl OperationLogLayer {
    pub fn new(state: AppContext) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for OperationLogLayer {
    type Service = OperationLogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            inner,
            state: self.state.clone(),
        }
    }
}

impl MiddlewareLayer for OperationLogLayer {
    fn name(&self) -> &'static str {
        "operation_log"
    }

    /// Returns whether the middleware is enabled or not
    fn is_enabled(&self) -> bool {
        true
    }

    fn config(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value({})
    }

    /// Applies the OperationLog middleware to the given Axum router.
    fn apply(&self, app: AXRouter<AppContext>) -> Result<AXRouter<AppContext>> {
        Ok(app.layer(self.clone()))
    }
}

#[derive(Clone)]
pub struct OperationLogService<S> {
    inner: S,
    state: AppContext,
}

impl<S, B> Service<Request<B>> for OperationLogService<S>
where
    S: Service<Request<B>, Response = Response<Body>, Error = Infallible> + Clone + Send + 'static, /* Inner Service must return Response<Body> and never error */
    S::Future: Send + 'static,
    B: axum::body::HttpBody<Data = Bytes> + From<Body> + Send + 'static,
    B::Error: std::fmt::Display,
{
    // Response type is the same as the inner service / handler
    type Response = S::Response;
    // Error type is the same as the inner service / handler
    type Error = S::Error;
    // Future type is the same as the inner service / handler
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let state = self.state.clone();
        let clone = self.inner.clone();
        let start_time = Instant::now();

        // take the service that was ready
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            // Example of extracting JWT and checking roles
            let (parts, body) = req.into_parts();
            let (req_params, req_bytes) = get_req_body_data(body).await;
 
            let current_request = Request::from_parts(parts.clone(), Body::from(req_bytes).into());

            let request_id = match current_request.extensions().get::<LocoRequestId>() {
                Some(x) => String::from(x.get()),
                None => String::from(""),
            };

            let response = inner.call(current_request).await?;
            
            // 获取响应数据
            let res_string = match response.extensions().get::<ResJsonString>() {
                Some(x) => x.clone().0,
                None => String::from(""),
            };
            
            // 计算执行时间
            let execution_time = start_time.elapsed().as_millis() as i64;

            tokio::spawn(async move {
                if let Err(e) = add_operation_log(
                    parts,
                    state,
                    execution_time,
                    request_id,
                    req_params,
                    res_string,
                ).await {
                    println!("add_operation_log error: {:?}", e);
                }
            });

            Ok(response)
        })
    }
}

async fn add_operation_log(
    mut parts: Parts,
    state: AppContext,
    execution_time: i64,
    request_id: String,
    req_params: String,
    res_string: String,
) -> Result<()> {
    let uri = String::from(parts.uri.path());
    
    // 日志列表操作不记录
    if uri.starts_with("/v1/operation-log") {
        return Ok(());
    }

    let ctx: AppContext = AppContext::from_ref(&state);
    let default_settings = std::sync::Arc::new(Settings::default());
    let settings = match ctx.shared_store.get::<Arc<Settings>>() {
        Some(s) => s,
        None => default_settings,
    };
    let not_auth_uris = settings.public_apis.clone();

    // 白名单接口不记录日志
    if not_auth_uris.contains(&uri) {
        return Ok(());
    }

    let jwt = JWTWithUser::<UserModel>::from_request_parts(&mut parts, &state).await?;
    let client = ClientInfo::from_request_parts(&mut parts, &state).await?;
    let method = String::from(parts.method.as_str());

    // 获取用户信息
    let user = match UserModel::find_by_id(&ctx.db, jwt.user.uid).await {
        Ok(user) => user,
        Err(_) => UserModel::default(),
    };
    // 获取部门信息
    let dpt = match DepartmentModel::find_by_id(&ctx.db, user.dpt_id).await {
        Ok(dpt) => dpt,
        Err(_) => DepartmentModel::default(),
    };

    let log_entity = OperationLogActiveModel {
        id: ActiveValue::NotSet,
        time_id: Set(utc_now().and_utc().timestamp()),
        title: Set(request_id),
        business_type: Set(String::from("")),
        method: Set(String::from(method.as_str())),
        request_method: Set(String::from(method.as_str())),
        operator_type: Set(String::from("")),
        name: Set(String::from(user.username.as_str())),
        department_name: Set(String::from(dpt.name.as_str())),
        url: Set(uri),
        ip: Set(client.ip),
        location: Set(client.location),
        param: Set(if req_params.len() > 10000 { "数据太长不保存".to_string() } else { req_params }),
        path_param: Set(String::from("")),
        json_result: Set(if res_string.len() > 65535 { "数据太长不保存".to_string() } else { res_string }),
        status: Set(String::from("1")),
        error_message: Set(String::from("")),
        duration: Set(execution_time),
        created_at: Set(utc_now()),
    };

    if method.eq("OPTIONS") {
        return Ok(());
    }

    // 保存到数据库
    OperationLogEntity::insert(log_entity)
        .exec(&ctx.db)
        .await?;

    Ok(())
}

/// 获取body数据
async fn get_req_body_data<B>(body: B) -> (String, Bytes)
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_err) => Bytes::new(),
    };

    let body_data = match std::str::from_utf8(&bytes) {
        Ok(x) => x.to_string(),
        Err(_) => "该数据无法转输出，可能为blob，binary".to_string(),
    };

    (body_data, bytes)
}
