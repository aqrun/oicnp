use std::time::Duration;
use reqwest::Client;
use loco_rs::{
    Result,
    Error,
};
use oic_core::{typings::JsonRes, AppContext};
use serde::Serialize;
use serde_json::Value;

/**
 * 生成API链接
 */
pub fn r(ctx: &AppContext, uri: &str) -> String {
    let host = ctx.config.server.host.as_str();
    let port = ctx.config.server.port;
    format!("{host}:{port}{uri}")
}

/// post 请求
pub async fn post(url: &str, params: &impl Serialize) -> Result<JsonRes<Value>> {
    let client = Client::new();
    let res = client.post(url)
        .json(params)
        .send()
        .await;

    let res = match res {
        Ok(res) => {
            if res.status().is_success() {
                let a = res.json::<JsonRes<Value>>()
                    .await
                    .map_err(|err| {
                        Error::BadRequest(format!("1-{err}"))
                    })?;
                a
            } else {
                return Err(Error::BadRequest(
                    format!("接口请求失败, 错误码：{}，URL：{}",
                    res.status(),
                    res.url()
                )));
            }
        },
        Err(err) => {
            return Err(Error::BadRequest(format!("{err}")));
        },
    };

    Ok(res)
}


/**
 * 格式化时间耗时
 * 秒级直接显示 1秒
 * 分钟级显示 1分钟1秒
 * 小时级显示 1小时1分钟1秒
 */
pub fn format_duration(duration: Duration) -> String {
    if duration.as_secs() < 60 {
        return format!("{}秒", duration.as_secs());
    } else if duration.as_secs() < 3600 {
        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;
        return format!("{}分钟{}秒", minutes, seconds);
    } else {
        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() % 3600) / 60;
        let seconds = duration.as_secs() % 60;
        return format!("{}小时{}分钟{}秒", hours, minutes, seconds);
    }
}