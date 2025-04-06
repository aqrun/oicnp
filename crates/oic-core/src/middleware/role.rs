use std::{
    convert::Infallible,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    response::Response,
};
use futures_util::future::BoxFuture;
use loco_rs::prelude::*;
use super::auth::JWTWithUser;
use tower::{Layer, Service};
use crate::entities::prelude::*;
use crate::typings::JsonRes;

#[derive(Clone)]
pub struct RoleRouteLayer {
    state: AppContext,
}

impl RoleRouteLayer {
    pub fn new(state: AppContext) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for RoleRouteLayer {
    type Service = RoleRouteService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            inner,
            state: self.state.clone(),
        }
    }
}
#[derive(Clone)]
pub struct RoleRouteService<S> {
    inner: S,
    state: AppContext,
}

impl<S, B> Service<Request<B>> for RoleRouteService<S>
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
        // take the service that was ready
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            // Example of extracting JWT and checking roles
            let (mut parts, body) = req.into_parts();
            // 当前 URL /v1/info
            let uri = String::from(parts.uri.path());

            // 是否拥有访问权限检测标识
            let mut has_permission = false;

            let not_auth_uris = vec![
                "/v1/info",
            ];

            // 解析当前登录用户信息
            let auth = match JWTWithUser::<UserModel>::from_request_parts(&mut parts, &state).await {
                Ok(auth) => auth,
                Err(_) => {
                    return Ok(no_auth("请先登录"));
                },
            };
            // 检测登录状态
            if auth.claims.uuid.is_empty() {
                return Ok(no_auth("请先登录"));
            }

            if auth.user.is_admin.eq("1") {
                // 管理员账号拥有所有权限
                has_permission = true;
            } else if not_auth_uris.contains(&uri.as_str()) {
                // 不需要权限检测
                has_permission = true;
            }

            if has_permission {
                let req = Request::from_parts(parts, body);
                inner.call(req).await
            } else {
                Ok(no_auth(""))
            }
        })
    }
}

fn no_auth(msg: &str) -> Response {
    let mut valid_msg = String::from("无权限访问");

    if !msg.is_empty() {
        valid_msg = String::from(msg);
    }

    JsonRes::<String>::code("401", valid_msg.as_str()).into_response()
}
