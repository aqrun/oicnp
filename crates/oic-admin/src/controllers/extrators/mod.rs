//! 从 `SESSIONID` Cookie 解析 session，再用 `admin:session:{id}` 从缓存读取 access token。

use axum::extract::{FromRef, FromRequestParts};
use axum::http::header::COOKIE;
use axum::http::request::Parts;
use cookie::Cookie;
use serde::{Deserialize, Serialize};

use crate::{WebAppContext, SESSION_ID};

/// 与 `services/auth.rs` 登录写入 Redis 的 key 规则一致。
#[must_use]
pub fn admin_session_cache_key(session_id: &str) -> String {
	format!("admin:session:{}", session_id.trim())
}

///
/// 获取当前 session_id，再从 cache 中获取 token。
/// 无 Cookie / 无缓存 / 读缓存失败时 `token` 为空字符串（便于 handler 自行返回 401）。
///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthToken {
	pub token: String,
}

impl<S> FromRequestParts<S> for AuthToken
where
	WebAppContext: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = std::convert::Infallible;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let ctx = WebAppContext::from_ref(state);

		let Some(session_id) = session_id_from_cookie(parts) else {
			return Ok(AuthToken {
				token: String::new(),
			});
		};

		if session_id.is_empty() {
			return Ok(AuthToken {
				token: String::new(),
			});
		}

		let key = admin_session_cache_key(&session_id);
		let token = match ctx.cache.get(&key).await {
			Ok(Some(t)) => t,
			Ok(None) => String::new(),
			Err(e) => {
				tracing::warn!(error = %e, key = %key, "auth token cache get failed");
				String::new()
			},
		};

		Ok(AuthToken { token })
	}
}

fn session_id_from_cookie(parts: &Parts) -> Option<String> {
	let raw = parts.headers.get(COOKIE)?.to_str().ok()?;
	for parsed in Cookie::split_parse(raw) {
		let c = parsed.ok()?;
		if c.name() == SESSION_ID {
			return Some(c.value().to_string());
		}
	}
	None
}
