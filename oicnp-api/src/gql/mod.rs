pub mod mutations;
pub mod node_query;
pub mod queries;
pub mod user_query;

pub use mutations::*;
pub use node_query::*;
pub use queries::*;
pub use user_query::*;

use crate::typings::{State, Token};
use crate::extensions::Auth as AuthExt;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use oicnp_core::{establish_connection, DB, G};
use poem::{
    handler,
    http::HeaderMap,
    web::{Data, Html},
    IntoResponse,
};

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let db = DB.get_or_init(establish_connection).await;

    Schema::build(QueryRoot::default(), MutationRoot, EmptySubscription)
        .data(db.clone())
        .extension(AuthExt)
        .finish()
}

#[handler]
pub async fn graphql(
    data: Data<&State>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;
    let schema = data.0.schema.clone();

    if let Some(token) = get_token_from_headers(headers) {
        req = req.data(token);
    } else {
        req = req.data(Token(String::from("Anonymous")));
    }

    schema.execute(req).await.into()
}

#[handler]
pub fn graphiql() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        &G.graphql_url,
    )))
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<Token> {
    headers
        .get("Token")
        .and_then(|value| {
            value.to_str().map(|s| {
                // Bearer [token]  形式分割获取后一部分
                let mut auth_arr = s.split(" ").collect::<Vec<&str>>();
                Token(auth_arr.pop().unwrap_or("").to_string())
            }).ok()
        })
}
