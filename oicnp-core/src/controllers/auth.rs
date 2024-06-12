use crate::models::{LoginInfo, ReqCtx};
use async_graphql::{self, Context, Object};
use crate::{
    prelude::anyhow::Result,
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
            token: String::from(""),
            uid: 0,
            role: String::from(""),
            exp: 0,
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
        let info = create_jwt(0, "Admin")?;
        Ok(info)
    }

    ///
    /// 账号登出
    ///
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let req_ctx = ctx.data_unchecked::<ReqCtx>();
        println!("req-ctx-----{:?}", req_ctx);
        Ok(true)
    }

    /// 用户邮箱验证，未验证的邮箱不能登陆系统
    async fn verify(&self) -> Result<&str> {
        Ok("")
    }

    /// 如果用户遗忘了密码，调用此接口会给用户邮箱发送重置链接
    async fn forgot(&self) -> Result<&str> {
        Ok("")
    }

    /// 根据参数重置用户密码
    async fn reset(&self) -> Result<&str> {
        Ok("")
    }
}

