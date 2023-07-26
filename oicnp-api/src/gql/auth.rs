use crate::models::{LoginInfo, ReqCtx};
use async_graphql::{Context, Object};
use oicnp_core::{
    models::auth::LoginInfo as CoreLoginInfo,
    prelude::anyhow::{anyhow, Result},
    services::auth::create_jwt,
};

#[derive(Default)]
pub struct AuthMutations;

#[Object]
impl AuthMutations {
    ///
    /// 注册账户
    ///
    async fn register(&self) -> Result<LoginInfo> {
        Ok(LoginInfo {
            data: CoreLoginInfo {
                token: String::from(""),
                uid: String::from(""),
                role: String::from(""),
                exp: 0,
            },
        })
    }

    ///
    /// 账号登陆
    ///
    async fn login(
        &self,
        username: Option<String>,
        email: Option<String>,
        password: String,
    ) -> Result<LoginInfo> {
        let info = create_jwt("abc", "Admin")?;
        let res = LoginInfo { data: info };
        Ok(res)
    }

    ///
    /// 账号登出
    ///
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let req_ctx = ctx.data_unchecked::<ReqCtx>();
        println!("req-ctx-----{:?}", req_ctx);
        Ok(true)
    }
}

