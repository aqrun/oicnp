use std::{
    convert::Infallible,
    task::{Context, Poll},
    time::Instant,
    sync::Arc,
};

use axum::{
    body::Body,
    extract::{FromRef, FromRequestParts, Request},
    response::Response,
    http::request::Parts,
};
use futures_util::future::BoxFuture;
use loco_rs::prelude::*;
use tower::{Layer, Service};
use crate::{
    entities::prelude::*,
    utils::utc_now,
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
#[derive(Clone)]
pub struct OperationLogService<S> {
    inner: S,
    state: AppContext,
}

impl<S, B> Service<Request<B>> for OperationLogService<S>
where
    S: Service<Request<B>, Response = Response<Body>, Error = Infallible> + Clone + Send + 'static, /* Inner Service must return Response<Body> and never error */
    S::Future: Send + 'static,
    B: Send + 'static,
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
            let response = inner.call(Request::from_parts(parts.clone(), body)).await?;
            
            // 计算执行时间
            let execution_time = start_time.elapsed().as_millis() as i64;

            tokio::spawn(async move {
                if let Err(e) = add_operation_log(
                    parts,
                    state,
                    execution_time,
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
    execution_time: i64
) -> Result<()> {
    let ctx: AppContext = AppContext::from_ref(&state);
    let default_settings = std::sync::Arc::new(Settings::default());
    let settings = match ctx.shared_store.get::<Arc<Settings>>() {
        Some(s) => s,
        None => default_settings,
    };

    let jwt = JWTWithUser::<UserModel>::from_request_parts(&mut parts, &state).await?;
    let client = ClientInfo::from_request_parts(&mut parts, &state).await?;
    let uri = String::from(parts.uri.path());
    let method = String::from(parts.method.as_str());

    let log_entity = OperationLogActiveModel {
        id: ActiveValue::NotSet,
        time_id: Set(utc_now().and_utc().timestamp()),
        title: Set(String::from("")),
        business_type: Set(String::from("")),
        method: Set(method.clone()),
        request_method: Set(method),
        operator_type: Set(String::from("")),
        name: Set(jwt.user.username),
        department_name: Set(String::from("")),
        url: Set(uri),
        ip: Set(client.ip),
        location: Set(client.location),
        param: Set(String::from("")),
        path_param: Set(String::from("")),
        json_result: Set(String::from("")),
        status: Set(String::from("1")),
        error_message: Set(String::from("")),
        duration: Set(execution_time),
        created_at: Set(utc_now()),
    };

    // 保存到数据库
    OperationLogEntity::insert(log_entity)
        .exec(&ctx.db)
        .await?;

    Ok(())
}
