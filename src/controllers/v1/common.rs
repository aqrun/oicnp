use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    utils::{get_api_prefix, get_auth_captcha, AuthCaptcha},
    typings::JsonRes,
    AppContext,
    services::cache::{RedisPool, Redis},
};
use std::time::Duration;

#[debug_handler]
pub async fn info(
    State(ctx): State<AppContext>,
) -> JsonRes<String> {
    let redis = match Redis::from(&ctx).await {
        Ok(redis) => redis,
        Err(e) => {
            return JsonRes::err(e.to_string());
        },
    };

    let a = redis.set("test", "test123").await;
    println!("{:?}", a);

    let a = redis.get("test").await;
    println!("{:?}", a);
    

    JsonRes::ok(String::from("Admin Api success"))
}

#[debug_handler]
pub async fn captcha(
    State(ctx): State<AppContext>,
) -> JsonRes<AuthCaptcha> {
    let captcha = get_auth_captcha();

    // 缓存验证码 10 分钟
    match ctx.cache.insert_with_expiry(
        captcha.id.as_str(),
        captcha.text.as_str(),
        Duration::from_secs(60 * 10),
    ).await {
        Ok(_) => {
            JsonRes::from((captcha.data(), "captcha"))
        },
        Err(err) => {
            JsonRes::err(err)
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "").as_str())
        .add("/info", get(info).post(info))
        .add("/captcha", get(captcha))
}