use std::sync::Arc;
use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::{
    utils::{get_api_prefix, get_auth_captcha, AuthCaptcha},
    typings::JsonRes,
    AppContext,
    services::cache::OicCache,
};
use std::time::Duration;

#[debug_handler]
pub async fn info(
    State(_ctx): State<AppContext>,
) -> JsonRes<String> {
    JsonRes::ok(String::from("Admin Api success"))
}

#[debug_handler]
pub async fn captcha(
    State(ctx): State<AppContext>,
) -> JsonRes<AuthCaptcha> {
    let cache = match ctx.shared_store.get::<Arc<OicCache>>() {
        Some(cache) => cache,
        None => {
            return JsonRes::err(String::from("Cache not found"));
        },
    };

    let captcha = get_auth_captcha();

    // 缓存验证码 10 分钟
    match cache.insert_with_expiry(
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