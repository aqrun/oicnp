use crate::typings::Token;
use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextParseQuery},
    parser::types::ExecutableDocument,
    ServerError, ServerResult, Variables,
};
use oicnp_core::{
    prelude::anyhow::{anyhow, Result},
    services::auth::decode_jwt,
};
use regex::Regex;
use std::sync::Arc;

/// Auth extension
#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
pub struct Auth;

impl ExtensionFactory for Auth {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthExtension)
    }
}

struct AuthExtension;

#[async_trait::async_trait]
impl Extension for AuthExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let token = ctx.data_unchecked::<Token>();
        let token_jwt = token.0.as_str();

        let document = next.run(ctx, query, variables).await?;

        let auth_res = check_auth(query, token_jwt);

        match auth_res {
            Ok(_) => Ok(document),
            Err(err) => {
                // 用户信息解析失败 返回错误信息
                Err(ServerError::new(format!("{:?}", err), None))
            }
        }
    }
}

fn check_auth(query: &str, auth_token: &str) -> Result<String> {
    let query = trim_query(query);

    let is_public_query = check_is_public_query(query.as_str());

    if is_public_query {
        return Ok(String::from("Authorized"));
    }

    // 不是公开接口 需要检测用户token的合法性
    let claims = decode_jwt(auth_token)?;

    if claims.uid.is_empty() {
        return Err(anyhow!("Anonymous"));
    }

    Ok(String::from("Authorized"))
}

fn trim_query(query: &str) -> String {
    let re = Regex::new(r"[\n\r\s]*").unwrap();
    let s = re.replace_all(query, "");
    String::from(s)
}

fn check_is_public_query(query: &str) -> bool {
    let public_auth_handles: Vec<&str> = vec!["register", "login", "IntrospectionQuery"];

    let target = public_auth_handles.into_iter().find(|item| {
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

