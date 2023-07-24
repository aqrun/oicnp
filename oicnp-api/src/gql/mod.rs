pub mod node;
mod roots;
pub mod user;

pub use node::*;
pub use roots::*;
pub use user::*;

// use crate::extensions::Auth as AuthExt;
use crate::typings::{State, Token};
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

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db.clone())
    // .extension(AuthExt)
    .finish()
}

#[handler]
pub async fn graphql(
    data: Data<&State>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.0;
    let schema = data.0.schema.clone();

    schema.execute(req).await.into()
}

#[handler]
pub fn graphiql() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        &G.graphql_url,
    )))
}
