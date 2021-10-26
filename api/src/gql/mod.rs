pub mod mutations;
pub mod queries;

pub use queries::*;
pub use mutations::*;

use poem::{IntoResponse, web::{Html, Data}, Endpoint, EndpointExt, handler};
use async_graphql_poem::GraphQL;
use async_graphql::{
    Schema, EmptySubscription,
    http::{
        GraphQLPlaygroundConfig, playground_source, receive_json,
    },
};
use crate::G;
use crate::typings::{State};
use crate::constants::{GRAPHIQL_PATH};
use crate::dbs::StarWars;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    // TODO: init by real database
    let database = StarWars::new();

    Schema::build(
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    )
        .data(database)
        .finish()
}

#[handler]
pub async fn graphql(data: Data<State>) -> GraphQL<QueryRoot, MutationRoot, EmptySubscription> {
    let schema = data.0.schema.clone();
    GraphQL::new(schema)
}

#[handler]
pub async fn graphiql() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new(
            G.get(GRAPHIQL_PATH).unwrap()
        )
    ))
}



