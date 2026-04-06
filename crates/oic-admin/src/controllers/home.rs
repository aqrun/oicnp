use axum::{
    Router,
    body::Bytes,
    routing::{get, post},
    extract::{State, Path},
    response::{IntoResponse, Redirect},
    http::{HeaderMap, Method, Uri, StatusCode},
    Json,
    debug_handler,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use crate::views::{
    render_home_index,
};
use super::extrators::AuthToken;
use crate::services::{
    fetch_auth_info,
    login as auth_login_service,
    menu::get_routes,
};
use crate::{cached, WebAppContext, SESSION_ID};
use oic_core::typings::JsonRes;
use oic_core::models::users::LoginParams;

pub async fn home_index(
    State(ctx): State<WebAppContext>,
    cookies: CookieJar,
    uri: Uri,
) -> impl IntoResponse {
    let uri_path = String::from(uri.path());
    let is_public_uri = ctx.config.admin.public_uri.contains(&uri_path);

    if !is_public_uri {
        let mut seccion_id = String::from("");

        if let Some(cookie) = cookies.get(SESSION_ID) {
            seccion_id = cookie.value().to_string();
        }

        if seccion_id.is_empty() {
            return Redirect::temporary("/auth/login").into_response();
        }
    }
    
    cached!(
        &ctx.cache,
        "admin:home:index",
        render_home_index(&ctx),
        ctx.config.handler_cache_time
    )
}

/// 请求上游 `POST /api/auth/login`，成功后写 `SESSIONID` Cookie，并在 Redis 存 `session-{id}` → token。
async fn auth_login(
    State(ctx): State<WebAppContext>,
    jar: CookieJar,
    Json(params): Json<LoginParams>,
) -> impl IntoResponse {
    match auth_login_service(&ctx, params).await {
        Ok(outcome) => {
            let cookie = Cookie::build((SESSION_ID, outcome.session_id))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .build();
            let jar = jar.add(cookie);
            let body = JsonRes::ok("Login Success");
            (jar, body).into_response()
        }
        Err(e) => {
            let msg = e.to_string();
            JsonRes::<String>::code("400", msg.as_str()).into_response()
        }
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

///
/// 获取当前登陆用户信息（`SESSIONID` → Redis `admin:session:{id}` → Bearer）。
async fn auth_user_info(
    State(ctx): State<WebAppContext>,
    auth: AuthToken,
) -> impl IntoResponse {
    if auth.token.is_empty() {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    match fetch_auth_info(&ctx, &auth.token).await {
        Ok(json) => Json(json).into_response(),
        Err(_e) => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    }
}

async fn get_async_routes(
    State(ctx): State<WebAppContext>,
    auth: AuthToken,
) -> impl IntoResponse {
    if auth.token.is_empty() {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    match get_routes(&ctx, &auth.token).await {
        Ok(json) => JsonRes::ok(json).into_response(),
        Err(_) => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    }
}

/// 
/// 路由转发，将请求转发到 API 服务
/// 
/// api 路径由config获取 ctx.config.api_url
/// 请求的 header 添加 Authorization: Bearer <token>
/// 返回的response可以不用解析直接输出就行
/// 
#[debug_handler]
async fn api(
    State(ctx): State<WebAppContext>,
    Path(uri): Path<String>,
    auth: AuthToken,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let token = auth.token;

    if token.is_empty() {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let upstream = format!("{}/v1/{}", ctx.config.api_url.trim_end_matches('/'), uri);
    let client = reqwest::Client::new();
    let mut req = client.request(method, &upstream).body(body);
    req = req.header("Authorization", format!("Bearer {}", token));

    // 按需透传部分 header（不要原样全透，host/content-length 这类要避开）
    if let Some(v) = headers.get("content-type") {
        req = req.header("content-type", v);
    }

    let resp = match req.send().await {
        Ok(r) => r,
        Err(_) => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let status = resp.status();
    let resp_headers = resp.headers().clone();
    let resp_body = resp.bytes().await.unwrap_or_default();
    (status, resp_headers, resp_body).into_response()
}

pub fn home_routes() -> Router<WebAppContext> {
    Router::new()
        .route("/", get(home_index))
        .route("/api/auth/login", post(auth_login))
        .route("/api/auth/logout", post(auth_logout))
        .route("/api/auth/user-info", get(auth_user_info).post(auth_user_info))
        .route("/api/get-async-routes", get(get_async_routes))
        .route("/api/{*uri}", get(api).post(api))
}