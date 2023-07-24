use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result, Response, Body};
use crate::models::auth::ReqCtx;
use oicnp_core::{
    G,
};

/// 菜单授权中间件
#[derive(Clone, Debug)]
pub struct Auth;

impl<E: Endpoint> Middleware<E> for Auth {
    type Output = AuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthEndpoint { ep }
    }
}

pub struct AuthEndpoint<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthEndpoint<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");

        // 是公开的数据接口直接放行 如 login register
        if ctx.gql_is_public_query {
            return self.ep.call(req).await;
        }

        // 如果是超级用户，则不需要验证权限，直接放行
        if !ctx.login_info.uid.is_empty()
            && G.super_user.contains(&ctx.login_info.uid)
        {
            return self.ep.call(req).await;
        }

        // 用户存在且不是非法用户直接放行
        if !ctx.login_info.uid.is_empty()
            && !ctx.login_info.role.eq("Anonymous")
        {
            return self.ep.call(req).await;
        }

        let body = Body::from_json(serde_json::json!({
            "code": "403",
            "message": "你没有权限访问该API"
        })).unwrap();
        let err_response = Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header("Content-Type", "application/json; charset=utf-8")
            .body(body);

        // 验证api权限，如果不在路由表中，则放行，否则验证权限

        // if ApiUtils::is_in(&ctx.path).await {
        //     if ApiUtils::check_api_permission(&ctx.path, &ctx.method, &ctx.user.id).await {
        //         return self.ep.call(req).await;
        //     } else {
        //         return Err(Error::from_string("你没有权限访问该页面/API", StatusCode::FORBIDDEN));
        //     }
        // } else {
        //     return self.ep.call(req).await;
        // }

        return Err(Error::from_response(err_response));
    }
}
