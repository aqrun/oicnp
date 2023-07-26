mod auth;
pub mod node;
mod roots;
pub mod user;

pub use auth::*;
pub use node::*;
pub use roots::*;
pub use user::*;

// use crate::extensions::Auth as AuthExt;
use crate::{models::ReqCtx, typings::State};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use oicnp_core::{establish_connection, DB, G};
use poem::{
    handler,
    web::{Data, Html},
    IntoResponse, Request,
};

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let db = DB.get_or_init(establish_connection).await;

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db.clone())
    .finish()
}

#[handler]
pub async fn graphql(
    data: Data<&State>,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut gql_req = gql_req.0;
    let schema = data.0.schema.clone();

    // 将 poem 中生成请求上下文转入 graphql
    if let Some(req_ctx) = data.0.req_ctx.clone() {
        gql_req = gql_req.data(req_ctx);
    }

    schema.execute(gql_req).await.into()
}

#[handler]
pub fn graphiql() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        &G.graphql_url,
    )))
}

