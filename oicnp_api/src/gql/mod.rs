pub mod mutations;
pub mod queries;
pub mod user_query;
pub mod node_query;

pub use queries::*;
pub use mutations::*;
pub use user_query::*;
pub use node_query::*;

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
use crate::typings::{State, GqlState};
use crate::services::G;
use crate::dbs::init_rbatis;
use rbatis::rbatis::Rbatis;
use std::sync::Arc;

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

pub async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    let rbatis: Rbatis = init_rbatis().await;
    let rbatis: Arc<Rbatis> = Arc::new(rbatis);
    let gql_state = GqlState{
        rbatis,
    };

    Schema::build(
        QueryRoot::default(),
        MutationRoot,
        EmptySubscription,
    )
        .data(gql_state)
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



