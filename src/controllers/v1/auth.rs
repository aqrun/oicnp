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
    middleware::{JWTWithUser, ClientInfo},
    constants::{LOGIN_EXPIRE_TIME, LOGIN_REMEMBER_EXPIRE_TIME},
};
use oic_core::services::{
    self,
    auth::{
        VerifyParams,
        ForgotParams,
        ResetParams,
        LoginResponse,
        RefreshTokenParams,
    },
    user::{add_user_login_log, update_user_online_log},
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
    let log_email = String::from(params.email.as_str());
    let mut login_status = true;
    let mut login_message = String::from("登陆成功");

    let mut info = LoginResponse::default();
    
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
            login_status = false;
            login_message = String::from("验证码已过期, 刷新后重试");
            String::from("")
        }
    };

    let valid_cache = cache_captcha.to_lowercase();
    let valid_captcha = params.captcha.to_lowercase();

    if !valid_cache.eq(valid_captcha.as_str()) {
        login_status = false;
        login_message = String::from("验证码错误");
    }

    if login_status {
        // 验证成功后删除缓存中的验证码，防止重复使用
        let _ = cache.remove(params.captcha_id.as_str()).await;
    
        info = match services::auth::login(&ctx.db, params).await {
            Ok(res) => res,
            Err(err) => {
                login_status = false;
                login_message = err.to_string();
                LoginResponse::default()
            }
        };
    
        let cache_key = format!("session-{}", info.token);
        let duration = if remember {
            // 7 days
            Duration::from_secs(LOGIN_REMEMBER_EXPIRE_TIME)
        } else {
            Duration::from_secs(LOGIN_EXPIRE_TIME)
        };
        
        if login_status {
            if let Err(err) = cache.insert_with_expiry(cache_key.as_str(), &info, duration).await {
                login_status = false;
                login_message = err.to_string();
            }
        }
    }

    let log_login_message = String::from(login_message.as_str());
    let log_login_response = info.clone();
    
    tokio::spawn(async move {
        let client = match ClientInfo::from_headers(&ctx, &headers).await {
            Ok(client) => client,
            Err(_err) => ClientInfo::default(),
        };
        let _ = UserModel::update_last_login_info(&ctx.db, info.uid, headers).await;

        // 添加登陆日志
        if let Err(e) = add_user_login_log(
            &ctx.db,
            client.clone(),
            log_email,
            login_status,
            log_login_message,
            String::from("auth/login"),
        ).await {
            println!("add_user_login_log error: {:?}", e);
        }

        // 更新用户在线日志
        if let Err(e) = update_user_online_log(
            &ctx.db,
            client,
            log_login_response,
        ).await {
            println!("update_user_online_log error: {:?}", e);
        }
    });

    if login_status {
        JsonRes::ok(info)
    } else {
        JsonRes::err(login_message)
    }
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

    let log_email = String::from(params.email.as_str());

    let mut login_status = true;
    let mut login_message = String::from("登陆成功");

    let info = match services::auth::access_token(
        &ctx.db, 
        cache,
        &ctx.config, 
        params,
    ).await {
        Ok(res) => res,
        Err(err) => {
            login_status = false;
            login_message = err.to_string();
            LoginResponse::default()
        }
    };

    let log_login_message = String::from(login_message.as_str());
    let log_login_response = info.clone();
    
    tokio::spawn(async move {
        let client = match ClientInfo::from_headers(&ctx, &headers).await {
            Ok(client) => client,
            Err(_err) => ClientInfo::default(),
        };
        let _ = UserModel::update_last_login_info(&ctx.db, info.uid, headers).await;

        if let Err(e) = add_user_login_log(
            &ctx.db,
            client.clone(),
            log_email,
            login_status,
            log_login_message,
            String::from("auth/access_token"),
        ).await {
            println!("add_user_login_log error: {:?}", e);
        }

        // 更新用户在线日志
        if let Err(e) = update_user_online_log(
            &ctx.db,
            client,
            log_login_response,
        ).await {
            println!("update_user_online_log error: {:?}", e);
        }
    });

    if login_status {
        JsonRes::ok(info)
    } else {
        JsonRes::err(login_message)
    }
}

#[debug_handler]
async fn auth_info(
    State(ctx): State<AppContext>,
    auth: JWTWithUser<UserModel>,
) -> JsonRes<LoginResponse> {
    let cache = match ctx.shared_store.get::<Arc<OicCache>>() {
        Some(cache) => cache,
        None => {
            return JsonRes::err(String::from("Cache not found"));
        },
    };
   
    let res = match services::auth::auth_info(&ctx.db, cache, &auth.user).await {
        Ok(res) => res,
        Err(err) => {
            return JsonRes::err(err.to_string());
        },
    };

    JsonRes::ok(res)
}

///
/// 优化点：
/// 1 refresh_token 参数未做基础校验（空值、长度、格式）
/// 2 绑定维度较弱：缓存只存 user_uuid，没绑定设备/客户端指纹；如果同账号多端，你目前是“任一 refresh token 都可换新”。
/// 
async fn handle_refresh_token(
    State(ctx): State<AppContext>,
    Json(params): Json<RefreshTokenParams>
) -> JsonRes<LoginResponse> {
    let cache = match ctx.shared_store.get::<Arc<OicCache>>() {
        Some(cache) => cache,
        None => {
            return JsonRes::err(String::from("Cache not found"));
        },
    };

    let info = match services::auth::refresh_token(
        &ctx.db, 
        cache,
        &ctx.config, 
        &params,
    ).await {
        Ok(res) => res,
        Err(_err) => {
            return JsonRes::err(String::from("刷新令牌失败"));
        }
    };

    JsonRes::ok(info)
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
        .add("/refresh-token", post(handle_refresh_token))
        .add("/info", post(auth_info))
}
