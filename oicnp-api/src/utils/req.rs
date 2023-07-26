use oicnp_core::{
    prelude::anyhow::{anyhow, Result},
    services::auth::decode_jwt,
    G,
};
use poem::Request;
use regex::Regex;

/// 解析头部的 Authorization: "Bearer [token]"
pub fn get_request_auth_token(req: &Request) -> String {
    let auth_token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or("");

    match auth_token {
        Ok(token) => token.replace("Bearer ", ""),
        _ => String::from(""),
    }
}

/// 去除请求体中的换行和空格
pub fn trim_gql_query(query: &str) -> String {
    let re = Regex::new(r"[\n\r\s]*").unwrap();
    let s = re.replace_all(query, "");
    String::from(s)
}

/// 检测接口权限
pub fn check_auth(query: &str, auth_token: &str) -> Result<String> {
    let query = trim_gql_query(query);

    let is_public_query = check_is_public_query(query.as_str());

    if is_public_query {
        return Ok(String::from("Authorized"));
    }

    // 不是公开接口 需要检测用户token的合法性
    let claims = decode_jwt(auth_token, !is_public_query)?;

    if claims.uid.is_empty() {
        return Err(anyhow!("Anonymous"));
    }

    Ok(String::from("Authorized"))
}

/// 当前请求是否为公开接口
pub fn check_is_public_query(query: &str) -> bool {
    let public_auth_handles: &[String] = G.white_list_api.as_slice();

    let target = public_auth_handles.iter().find(|item| {
        // 没有名字的匹配前缀
        let query_prefix = format!("{{{}(", item);
        // 有名字的匹配前缀
        let named_query_prefix = format!("query{}{{", item);
        let mutation_prefix = format!("mutation{{{}", item);

        if query.starts_with(mutation_prefix.as_str()) {
            return true;
        }

        if query.starts_with(named_query_prefix.as_str()) {
            return true;
        }

        if query.starts_with(query_prefix.as_str()) {
            return true;
        }

        return false;
    });

    // 可以匹配表示是公开接口不需要权限检查
    if let Some(_target) = target {
        return true;
    }

    false
}
