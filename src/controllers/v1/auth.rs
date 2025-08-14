use std::sync::Arc;
use std::time::Duration;
use axum::{
    debug_handler,
    http::HeaderMap,
};
use loco_rs::prelude::*;
use oic_core::{
    models::users::{LoginParams, RegisterParams, UserModel},
    typings::JsonRes,
    utils::get_api_prefix,
    services::cache::OicCache,
    middleware::JWTWithUser,
    auth::UserClaims,
};
use oic_core::services::{
    self,
    auth::{
        VerifyParams,
        ForgotParams,
        ResetParams,
        LoginResponse,
    },
};
use serde_json::Value;

// use crate::{
//     mailers::auth::AuthMailer,
// };

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> JsonRes<Value> {
    let res = services::auth::register(&ctx.db, params).await;
    JsonRes::from(res)
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyParams>,
) -> JsonRes<()> {
    let res = services::auth::verify(&ctx.db, params).await;
    JsonRes::from(res)
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> JsonRes<()> {
    let res = services::auth::forgot(&ctx.db, params).await;

    JsonRes::from(res)
    // AuthMailer::forgot_password(&ctx, &user).await?;
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> JsonRes<()> {
    let res = services::auth::reset(&ctx.db, params).await;

    JsonRes::from(res)
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(
    headers: HeaderMap,
    State(ctx): State<AppContext>,
    Json(params): Json<LoginParams>,
) -> JsonRes<LoginResponse> {
    let remember = params.remember;
    let cache = match ctx.shared_store.get::<Arc<OicCache>>() {
        Some(cache) => cache,
        None => {
            return JsonRes::err(String::from("Cache not found"));
        },
    };

    let cache_captcha = match cache.get(params.captcha_id.as_str()).await {
        Ok(text) => text.unwrap_or(String::from("")),
        Err(_) => {
            return JsonRes::err("验证码已过期, 刷新后重试");
        }
    };

    let valid_cache = cache_captcha.to_lowercase();
    let valid_captcha = params.captcha.to_lowercase();

    if !valid_cache.eq(valid_captcha.as_str()) {
        return JsonRes::err("验证码错误");
    }

    // 验证成功后删除缓存中的验证码，防止重复使用
    let _ = ctx.cache.remove(params.captcha_id.as_str()).await;

    let info = match services::auth::login(&ctx.db, params).await {
        Ok(res) => res,
        Err(err) => {
            return JsonRes::err(err.to_string());
        }
    };

    let cache_key = format!("session-{}", info.token);
    let duration = if remember {
        // 7 days
        Duration::from_secs(60 * 60 * 24 * 7)
    } else {
        Duration::from_secs(60 * 60 * 24)
    };

    if let Err(err) = cache.insert_with_expiry(cache_key.as_str(), &info, duration).await {
        return JsonRes::err(err);
    }

    let _ = UserModel::update_last_login_info(&ctx.db, info.uid, headers).await;

    JsonRes::ok(info)
}

/// Creates a user login and returns a token
#[debug_handler]
async fn access_token(
    headers: HeaderMap,
    State(ctx): State<AppContext>,
    Json(params): Json<LoginParams>,
) -> JsonRes<LoginResponse> {
    let cache = match ctx.shared_store.get::<Arc<OicCache>>() {
        Some(cache) => cache,
        None => {
            return JsonRes::err(String::from("Cache not found"));
        },
    };

    let cache_captcha = match cache.get(params.captcha_id.as_str()).await {
        Ok(text) => text.unwrap_or(String::from("")),
        Err(_) => {
            return JsonRes::err("验证码已过期, 刷新后重试");
        }
    };

    let valid_cache = cache_captcha.to_lowercase();
    let valid_captcha = params.captcha.to_lowercase();

    if !valid_cache.eq(valid_captcha.as_str()) {
        return JsonRes::err("验证码错误");
    }

    // 验证成功后删除缓存中的验证码，防止重复使用
    let _ = ctx.cache.remove(params.captcha_id.as_str()).await;

    let info = match services::auth::login(&ctx.db, params).await {
        Ok(res) => res,
        Err(err) => {
            return JsonRes::err(err.to_string());
        }
    };

    let _ = UserModel::update_last_login_info(&ctx.db, info.uid, headers).await;

    JsonRes::ok(info)
}

#[debug_handler]
async fn auth_info(
    auth: JWTWithUser<UserModel>,
    State(_ctx): State<AppContext>,
) -> JsonRes<UserClaims> {
    JsonRes::from((auth.claims, "user"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "auth").as_str())
        .add("/register", post(register))
        .add("/verify", post(verify))
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/access-token", post(access_token))
        .add("/info", post(auth_info))
}
