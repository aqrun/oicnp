use anyhow::Result;
use oic_core::{
    models::{
        users::LoginParams,
    },
    services::auth::LoginResponse,
};
use oic_core::typings::JsonRes;
use super::{call_api_with_bearer, parse_response};
use crate::WebAppContext;

/// 获取当前登陆用户信息
pub async fn describe_user_info(
    ctx: &WebAppContext,
    bearer: &str,
) -> Result<JsonRes<LoginResponse>> {
    let url = format!("{}/v1/user/info", ctx.config.api_url);
    let json_value = call_api_with_bearer(&url, bearer, &LoginParams::default()).await?;
    parse_response(json_value)
}

///
/// 调用 access-token API，返回 JsonRes<LoginResponse>
/// 
pub async fn describe_auth_login(
    ctx: &WebAppContext,
    params: LoginParams,
) -> Result<JsonRes<LoginResponse>> {
    let url = format!("{}/v1/auth/access-token", ctx.config.api_url);
    let json_value = call_api_with_bearer(&url, "", &params).await?;
    parse_response(json_value)
}