use crate::typings::Token;
use crate::utils::{trim_gql_query, check_auth};
use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextParseQuery},
    parser::types::ExecutableDocument,
    ServerError, ServerResult, Variables,
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



