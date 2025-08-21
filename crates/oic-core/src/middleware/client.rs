use std::collections::HashMap;
use std::sync::Arc;
use std::borrow::Cow;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, HeaderMap},
};
use serde::{Deserialize, Serialize};

use loco_rs::{
  app::AppContext,
  errors::Error,
};
use crate::services::settings::Settings;
use user_agent_parser::UserAgentParser;

// ---------------------------------------
//
// ClientInfo extractor
//
// ---------------------------------------

// Define a struct to represent user authentication information serialized
// to/from JSON
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ClientInfo {
    pub ip: String,
    pub location: String,
    pub net_work: String,
    pub browser: String,
    pub os: String,
    pub device: String,
}

// Implement the FromRequestParts trait for the Auth struct
impl<S> FromRequestParts<S> for ClientInfo
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        let ctx: AppContext = AppContext::from_ref(state);
        let default_settings = std::sync::Arc::new(Settings::default());
        let settings = match ctx.shared_store.get::<Arc<Settings>>() {
            Some(s) => s,
            None => default_settings,
        };
        let user_agent_parser = settings.user_agent_parser.clone();

        let headers = &parts.headers;
        let user_agent = headers.get("user-agent").unwrap().to_str().unwrap();
        let ip = get_remote_ip(headers.clone());
        let ua = get_user_agent_info(user_agent, user_agent_parser.as_str());
        let net = get_city_by_ip(ip.as_str()).await.unwrap();
        
        let info = Self {
            ip: net.ip,
            location: net.location,
            net_work: net.net_work,
            browser: ua.browser,
            os: ua.os,
            device: ua.device,
        };

        Ok(info)
    }
}

pub fn get_remote_ip(header: HeaderMap) -> String {
    let ip = match header.get("X-Forwarded-For") {
        Some(x) => {
            let mut ips = x.to_str().unwrap().split(',');
            ips.next().unwrap().trim().to_string()
        }
        None => match header.get("X-Real-IP") {
            Some(x) => x.to_str().unwrap().to_string(),
            None => "".to_string(),
        },
    };
    ip
}

pub fn get_user_agent_info(user_agent: &str, user_agent_parser: &str) -> UserAgentInfo {
    let ua_parser = UserAgentParser::from_path(user_agent_parser).unwrap();
    let product_v = ua_parser.parse_product(user_agent);
    let os_v = ua_parser.parse_os(user_agent);
    let device_v = ua_parser.parse_device(user_agent);
    let browser = product_v.name.unwrap_or(Cow::Borrowed("")).to_string() + " " + product_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
    let os = os_v.name.unwrap_or(Cow::Borrowed("")).to_string() + " " + os_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
    let device = device_v.name.unwrap_or(Cow::Borrowed("")).to_string();
    UserAgentInfo {
        browser: browser.trim().to_string(),
        os: os.trim().to_string(),
        device,
    }
}

pub async fn get_city_by_ip(ip: &str) -> Result<ClientNetInfo, Box<dyn std::error::Error>> {
    let url = "http://whois.pconline.com.cn/ipJson.jsp?json=true&ip=".to_string() + ip;
    let resp = reqwest::get(url.as_str()).await?.text_with_charset("utf-8").await?;
    let res = serde_json::from_str::<HashMap<String, String>>(resp.as_str())?;
    let location = format!("{}{}", res["pro"], res["city"]);
    let net_work = res["addr"].split(' ').collect::<Vec<&str>>()[1].to_string();
    Ok(ClientNetInfo {
        ip: res["ip"].to_string(),
        location,
        net_work,
    })
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct UserAgentInfo {
    pub browser: String,
    pub os: String,
    pub device: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ClientNetInfo {
    pub ip: String,
    pub location: String,
    pub net_work: String,
}
