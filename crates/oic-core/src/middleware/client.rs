use std::sync::Arc;
use std::borrow::Cow;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, HeaderMap},
};
use serde::{Deserialize, Serialize};

use loco_rs::{
    prelude::*,
    app::AppContext,
    errors::Error,
};
use crate::{
    services::settings::Settings,
    entities::prelude::*,
    prelude::ModelCrudHandler,
    models::ips::CreateIpReqParams,
};
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
    pub network: String,
    pub browser: String,
    pub os: String,
    pub device: String,
}

impl ClientInfo {
    pub async fn from_headers(
        ctx: &AppContext,
        headers: &HeaderMap,
    ) -> Result<Self, Error> {
        let default_settings = std::sync::Arc::new(Settings::default());
        let settings = match ctx.shared_store.get::<Arc<Settings>>() {
            Some(s) => s,
            None => default_settings,
        };
        let user_agent_parser = settings.user_agent_parser.clone();

        let user_agent = headers.get("user-agent").unwrap().to_str().unwrap();
        let ip = get_remote_ip(headers.clone());
        let ua = get_user_agent_info(user_agent, user_agent_parser.as_str());
        let net = match get_net_info(&ctx.db, ip.as_str()).await {
            Ok(x) => x,
            Err(_) => ClientNetInfo::default()
        };
        
        let info = Self {
            ip: net.ip,
            location: net.location,
            network: net.network,
            browser: ua.browser,
            os: ua.os,
            device: ua.device,
        };

        Ok(info)
    }
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
        let headers = &parts.headers;
        
        Self::from_headers(&ctx, headers).await
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

pub async fn get_net_info(
    db: &DatabaseConnection,
    ip: &str,
) -> Result<ClientNetInfo, Box<dyn std::error::Error>> {
    if ip.is_empty() 
        || ip.eq("127.0.0.1")
        || ip.eq("::1")
        || ip.eq("localhost")
    {
        return Ok(ClientNetInfo::default());
    }

    if let Ok(x) = IpModel::find_by_vid(db, ip).await {
        let info = ClientNetInfo {
            ip: x.ip,
            province: String::from(x.province.as_str()),
            city: String::from(x.city.as_str()),
            province_code: String::from(x.province_code.as_str()),
            city_code: String::from(x.city_code.as_str()),
            region: String::from(x.region.as_str()),
            region_code: String::from(x.region_code.as_str()),
            region_names: String::from(x.region_names.as_str()),
            location: format!("{}{}{}", x.province.as_str(), x.city.as_str(), x.region.as_str()),
            network: String::from(x.network.as_str()),
            ..Default::default()
        };
        return Ok(info);
    }

    let info = get_city_by_ip(ip).await?;

    if !info.province.is_empty()
        && !info.city.is_empty()
        && !info.ip.is_empty()
    {
        let create_params = CreateIpReqParams {
            id: None,
            ip: Some(String::from(info.ip.as_str())),
            province: Some(String::from(info.province.as_str())),
            city: Some(String::from(info.city.as_str())),
            province_code: Some(String::from(info.province_code.as_str())),
            city_code: Some(String::from(info.city_code.as_str())),
            region: Some(String::from(info.region.as_str())),
            region_code: Some(String::from(info.region_code.as_str())),
            region_names: Some(String::from(info.region_names.as_str())),
            network: Some(String::from(info.network.as_str())),
            created_at: None,
        };
        let _ = IpModel::upsert(db, &create_params).await?;
    }

    Ok(info)
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

    let mut info = serde_json::from_str::<ClientNetInfo>(resp.as_str())?;
    info.location = format!("{}{}", info.province, info.city);
    info.network = info.address.split(' ').collect::<Vec<&str>>()[1].to_string();

    Ok(info)
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct UserAgentInfo {
    pub browser: String,
    pub os: String,
    pub device: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
#[serde(default)]
pub struct ClientNetInfo {
    pub ip: String,
    #[serde(rename(deserialize = "pro", serialize = "pro"))]
    pub province: String,
    pub city: String,
    #[serde(rename(deserialize = "proCode", serialize = "proCode"))]
    pub province_code: String,
    #[serde(rename(deserialize = "cityCode", serialize = "cityCode"))]
    pub city_code: String,
    pub region: String,
    #[serde(rename(deserialize = "regionCode", serialize = "regionCode"))]
    pub region_code: String,
    /// 陕西省西安市 电信ADSL
    #[serde(rename(deserialize = "addr", serialize = "addr"))]
    pub address: String,
    #[serde(rename(deserialize = "regionNames", serialize = "regionNames"))]
    pub region_names: String,
    pub err: String,
    /// 陕西省西安市
    pub location: String,
    pub network: String,
}

impl Default for ClientNetInfo {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            location: "陕西省西安市".to_string(),
            network: "电信".to_string(),
            province: "陕西省".to_string(),
            city: "西安市".to_string(),
            province_code: "610000".to_string(),
            city_code: "610100".to_string(),
            region: "".to_string(),
            region_code: "".to_string(),
            address: "".to_string(),
            region_names: "".to_string(),
            err: "".to_string(),
        }
    }
}
