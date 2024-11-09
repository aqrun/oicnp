use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use anyhow::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct YoudaoTranslateArgs {
    pub q: String,
    pub from: String,
    pub to: String,
    pub appKey: String,
    pub salt: String,
    pub sign: String,
    pub signType: String,
    pub curtime: String,
    pub ext: Option<String>,
    pub voice: Option<String>,
    pub strict: Option<String>,
    pub vocabld: Option<String>,
    pub domain: Option<String>,
    pub rejectFallback: Option<String>,
}


pub async fn youdao_translate(q: &str) -> Result<String> {
    // 应用ID
    let app_key = "";
    // 应用密钥
    let app_secret = "";
    // UUID
    let salt = Utc::now().timestamp();
    // 当前时间戳
    let curtime = Utc::now().timestamp();

    // appKey + input + salt + curtime + appSecret
    let sign_source = format!("{}{}{}{}{}", app_key, q, salt, curtime, app_secret);
    let mut hasher = Sha256::new();
    hasher.update(sign_source);
    let sign = format!("{:X}", hasher.finalize());

    let mut args = YoudaoTranslateArgs {
        q: String::from(q),
        from: String::from("zh-CHS"),
        to: String::from("en"),
        appKey: String::from(app_key),
        salt: format!("{}", salt),
        sign: String::from(sign.as_str()),
        signType: String::from("v3"),
        curtime: format!("{}", curtime),
        ext: None,
        voice: None,
        strict: None,
        vocabld: None,
        domain: None,
        rejectFallback: None,
    };

    let client = reqwest::Client::new();
    let res = client.post("https://openapi.youdao.com/api")
        .json(&args)
        .send()
        .await?;

    println!("trans: {:?}", res);
    Ok(String::from(""))
}