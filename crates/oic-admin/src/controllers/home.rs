use axum::{
    Router,
    body::Bytes,
    routing::{get, post},
    extract::{State, Path},
    response::{IntoResponse, Redirect},
    http::{HeaderMap, Method},
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use crate::views::{
    render_home_index,
};
use crate::services::{
    fetch_auth_info,
    login as auth_login_service,
    AuthInfoRequest,
    AuthLoginRequest,
};
use crate::{cached, WebAppContext, SESSION_ID};
use oic_core::typings::JsonRes;

pub async fn home_index(
    State(ctx): State<WebAppContext>,
    cookies: CookieJar,
) -> impl IntoResponse {
    let mut seccion_id = String::from("");

    if let Some(cookie) = cookies.get(SESSION_ID) {
        seccion_id = cookie.value().to_string();
    }

    if seccion_id.is_empty() {
        return Redirect::temporary("/auth/login").into_response();
    }

    cached!(
        &ctx.cache,
        "home:index",
        render_home_index(&ctx),
        ctx.config.handler_cache_time
    )
}

/// 请求上游 `POST /api/auth/login`，成功后写 `SESSIONID` Cookie，并在 Redis 存 `session-{id}` → token。
async fn auth_login(
    State(ctx): State<WebAppContext>,
    jar: CookieJar,
    Json(payload): Json<AuthLoginRequest>,
) -> impl IntoResponse {
    match auth_login_service(&ctx, payload).await {
        Ok(outcome) => {
            let cookie = Cookie::build((SESSION_ID, outcome.session_id))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .build();
            let jar = jar.add(cookie);
            (jar, Json(outcome.upstream_json)).into_response()
        }
        Err(e) => {
            let msg = e.to_string();
            JsonRes::<String>::code("400", msg.as_str()).into_response()
        }
    }
}

fn bearer_token(headers: &HeaderMap) -> Option<String> {
    let v = headers.get(axum::http::header::AUTHORIZATION)?.to_str().ok()?;
    let rest = v.strip_prefix("Bearer ").or_else(|| v.strip_prefix("bearer "))?;
    let t = rest.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

async fn auth_logout(jar: CookieJar) -> impl IntoResponse {
    let cleared = Cookie::build((SESSION_ID, ""))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(cookie::time::Duration::ZERO)
        .build();
    let jar = jar.add(cleared);
    (jar, JsonRes::ok(String::from("Admin Api success"))).into_response()
}

async fn auth_user_info(
    State(ctx): State<WebAppContext>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let Some(bearer) = bearer_token(&headers) else {
        return JsonRes::<String>::code("401", "Unauthorized").into_response();
    };

    let req: AuthInfoRequest = if body.is_empty() {
        AuthInfoRequest::default()
    } else {
        match serde_json::from_slice(&body) {
            Ok(r) => r,
            Err(_) => return JsonRes::<String>::code("400", "Invalid JSON body").into_response(),
        }
    };

    match fetch_auth_info(&ctx, &bearer, &req).await {
        Ok(json) => Json(json).into_response(),
        Err(e) => JsonRes::<String>::code("400", e.to_string().as_str()).into_response(),
    }
}

/// 
/// 路由转发，将请求转发到 API 服务
/// 
/// api 路径由config获取 ctx.config.api_url
/// 请求的 header 添加 Authorization: Bearer <token>
/// 返回的response可以不用解析直接输出就行
/// 
async fn api(
    State(ctx): State<WebAppContext>,
    Path(uri): Path<String>,
    cookies: CookieJar,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let session_id = match cookies.get(SESSION_ID) {
        Some(c) => c.value().to_string(),
        None => return JsonRes::<String>::code("401", "Unauthorized").into_response(),
    };

    let session_key = format!("session-{}", session_id);
    let token = match ctx.cache.get(&session_key).await {
        Ok(Some(t)) => t,
        Ok(None) => return JsonRes::<String>::code("401", "Unauthorized").into_response(),
        Err(_) => return JsonRes::<String>::code("500", "Internal server error").into_response(),
    };

    let upstream = format!("{}/api/{}", ctx.config.api_url.trim_end_matches('/'), uri);
    let client = reqwest::Client::new();
    let mut req = client.request(method, &upstream).body(body);
    req = req.header("Authorization", format!("Bearer {}", token));

    // 按需透传部分 header（不要原样全透，host/content-length 这类要避开）
    if let Some(v) = headers.get("content-type") {
        req = req.header("content-type", v);
    }

    let resp = match req.send().await {
        Ok(r) => r,
        Err(_) => return JsonRes::<String>::code("502", "Upstream error").into_response(),
    };

    let status = resp.status();
    let resp_headers = resp.headers().clone();
    let resp_body = resp.bytes().await.unwrap_or_default();
    (status, resp_headers, resp_body).into_response()
}

pub fn home_routes() -> Router<WebAppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(home_index))
        .route("/api/auth/login", post(auth_login))
        .route("/api/auth/logout", post(auth_logout))
        .route("/api/auth/user_info", post(auth_user_info))
        .route("/api/{*uri}", get(api).post(api))
}