use reqwest::Client;
use loco_rs::{
    Result,
    Error,
};
use oic_core::{typings::JsonRes, AppContext};
use serde::Serialize;

/**
 * 生成API链接
 */
pub fn r(ctx: &AppContext, uri: &str) -> String {
    let host = ctx.config.server.host.as_str();
    let port = ctx.config.server.port;
    format!("{host}:{port}{uri}")
}

/// post 请求
pub async fn post(url: &str, params: &impl Serialize) -> Result<JsonRes> {
    let client = Client::new();
    let res = client.post(url)
        .json(params)
        .send()
        .await;

    let res = match res {
        Ok(res) => {
            if res.status().is_success() {
                let a = res.json::<JsonRes>()
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