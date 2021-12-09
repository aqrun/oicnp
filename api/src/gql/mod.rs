pub mod mutations;
pub mod queries;
pub mod user_query;

pub use queries::*;
pub use mutations::*;
pub use user_query::*;

use poem::{
    IntoResponse, web::{Html, Data, Json}, handler
};
use async_graphql::{
    Schema, EmptySubscription,
    http::{
        GraphQLPlaygroundConfig, playground_source,
    },
    Request, Response,
};
use crate::typings::{State};
use crate::services::G;

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    // TODO: init by real database
    // let database = StarWars::new();

    Schema::build(
        QueryRoot::default(),
        MutationRoot,
        EmptySubscription,
    )
        .finish()
}

#[handler]
pub async fn graphql(data: Data<&State>, req: Json<Request>) -> Json<Response> {
    let schema = data.0.schema.clone();
    Json(schema.execute(req.0).await)
}

#[handler]
pub fn graphiql() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new(
            &G.config.graphql_url
        )
    ))
}



