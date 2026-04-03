//! 认证相关：转发上游 `/api/auth/login`，并在 Redis 写入 `session-{id}` → Bearer token。

use anyhow::{Context, Result};
use oic_core::models::users::LoginParams;
use oic_core::typings::JsonDataRes;
use oic_core::services::auth::LoginResponse;
use serde_json::Value;
use oic_core::uuid;
use crate::services::{describe_auth_login, describe_user_info};
use crate::WebAppContext;

/// 登录成功：用于写 Cookie 与返回上游 JSON。
#[derive(Debug, Default)]
pub struct AuthLoginOutcome {
	/// 写入 `SESSIONID` Cookie 的值；Redis 键为 `session-{session_id}`。
	pub session_id: String,
	pub ttl_secs: u64,
}

fn login_ttl_secs(remember: bool) -> u64 {
	if remember {
		7 * 24 * 3600
	}
	else {
		24 * 3600
	}
}

/// 调用上游 `POST {api_url}/api/auth/login`，成功后写入 Redis，并返回完整上游 JSON（供 BFF 原样返回给前端）。
pub async fn login(
    ctx: &WebAppContext,
    params: LoginParams,
) -> Result<AuthLoginOutcome> {
    let remember = params.remember;
	let auth_res = describe_auth_login(ctx, params).await?;

    if auth_res.data.is_none() {
        let mut msg = String::from("auth login failed");

        if let Some(message) = auth_res.message {
            msg = message;
        }

        return Err(anyhow::anyhow!(msg));
    }
    let res_data = auth_res.data.unwrap();
    let token = res_data.token;

	let ttl_secs = login_ttl_secs(remember);
    let session_id = uuid!();
	let session_key = format!("admin:session:{}", session_id.as_str());

	ctx
		.cache
		.set_ex(&session_key, &token, ttl_secs)
		.await
		.context("failed to store session in cache")?;

	Ok(AuthLoginOutcome {
        session_id,
        ttl_secs,
    })
}

/// 调用上游 `POST {api_url}/api/auth/info`，使用 Bearer，不访问本地 cache。
pub async fn fetch_auth_info(
	ctx: &WebAppContext,
	bearer: &str,
) -> Result<JsonDataRes<LoginResponse>> {
	let user_info = describe_user_info(ctx, bearer)
		.await
		.context("auth info failed")?;
	Ok(user_info)
}

pub async fn get_routes(
    ctx: &WebAppContext,
    bearer: &str,
) -> Result<Value> {
    let res = serde_json::json!([
        {
            "path": "/home",
            "component": "/home/index.tsx",
            "handle": {
                "icon": "HomeOutlined",
                "title": "common.menu.home",
                "order": 1,
            },
        },
        {
            "path": "/about",
            "component": "/about/index.tsx",
            "handle": {
                "icon": "CopyrightOutlined",
                "title": "common.menu.about",
                "order": 2,
            },
        },
    ]);
   
    Ok(res)
}
