//! 认证相关：转发上游 `/api/auth/login`，并在 Redis 写入 `session-{id}` → Bearer token。

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::services::{call_api, call_api_with_bearer};
use crate::WebAppContext;

/// 与 `bak/backend` DescribeLoginRequestParams 对齐；同时兼容前端仅传 `username` + `password`。
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthLoginRequest {
	#[serde(default)]
	pub username: Option<String>,
	#[serde(default)]
	pub email: Option<String>,
	pub password: String,
	#[serde(default)]
	pub remember: bool,
	#[serde(default)]
	pub captcha_id: Option<String>,
	#[serde(default)]
	pub captcha: Option<String>,
}

/// 发往上游的请求体（camelCase 与 TS 侧一致）。
#[derive(Serialize)]
struct AuthLoginUpstreamBody {
	email: String,
	password: String,
	remember: bool,
	#[serde(skip_serializing_if = "Option::is_none", rename = "captchaId")]
	captcha_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	captcha: Option<String>,
}

/// 登录成功：用于写 Cookie 与返回上游 JSON。
#[derive(Debug)]
pub struct AuthLoginOutcome {
	/// 写入 `SESSIONID` Cookie 的值；Redis 键为 `session-{session_id}`。
	pub session_id: String,
	pub ttl_secs: u64,
	pub upstream_json: Value,
}

fn resolve_email(req: &AuthLoginRequest) -> Result<String> {
	let email = req
		.email
		.clone()
		.or_else(|| req.username.clone())
		.map(|s| s.trim().to_string())
		.filter(|s| !s.is_empty());
	email.ok_or_else(|| anyhow::anyhow!("email or username is required"))
}

fn login_ttl_secs(remember: bool) -> u64 {
	if remember {
		30 * 24 * 3600
	}
	else {
		24 * 3600
	}
}

/// 调用上游 `POST {api_url}/api/auth/login`，成功后写入 Redis，并返回完整上游 JSON（供 BFF 原样返回给前端）。
pub async fn login(ctx: &WebAppContext, req: AuthLoginRequest) -> Result<AuthLoginOutcome> {
	let email = resolve_email(&req)?;
	let body = AuthLoginUpstreamBody {
		email,
		password: req.password,
		remember: req.remember,
		captcha_id: req.captcha_id.clone(),
		captcha: req.captcha.clone(),
	};

	let url = format!(
		"{}/api/auth/login",
		ctx.config.api_url.trim_end_matches('/')
	);

	let json_value = call_api::<AuthLoginUpstreamBody>(&url, &body)
		.await
		.with_context(|| format!("auth login failed: {url}"))?;

	let token = json_value
		.pointer("/data/token")
		.and_then(|v| v.as_str())
		.map(String::from)
		.context("upstream login response missing data.token")?;

	let ttl_secs = login_ttl_secs(req.remember);
	let session_id = Uuid::new_v4().to_string();
	let session_key = format!("session-{}", session_id);

	ctx
		.cache
		.set_ex(&session_key, &token, ttl_secs)
		.await
		.context("failed to store session in cache")?;

	Ok(AuthLoginOutcome {
		session_id,
		ttl_secs,
		upstream_json: json_value,
	})
}

/// 与 `bak/backend` DescribeAuthInfoRequestParams 对齐（可为 `{}`）。
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AuthInfoRequest {
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "_name")]
	pub name: Option<String>,
}

/// 调用上游 `POST {api_url}/api/auth/info`，使用 Bearer，不访问本地 cache。
pub async fn fetch_auth_info(
	ctx: &WebAppContext,
	bearer: &str,
	req: &AuthInfoRequest,
) -> Result<Value> {
	let url = format!(
		"{}/api/auth/info",
		ctx.config.api_url.trim_end_matches('/')
	);
	call_api_with_bearer(&url, bearer, req)
		.await
		.with_context(|| format!("auth info failed: {url}"))
}
