pub mod mutations;
pub mod queries;

pub use queries::*;
pub use mutations::*;

use tide::{http::mime, Request, Response, StatusCode, Body};
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

pub async fn graphql(req: Request<State>) -> tide::Result {
    let schema = req.state().schema.clone();
    let gql_resp = schema.execute(
        receive_json(req).await?
    ).await;

    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(Body::from_json(&gql_resp)?);

    Ok(resp.into())
}

pub async fn graphiql(_: Request<State>) -> tide::Result {
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(playground_source(
        GraphQLPlaygroundConfig::new(
            G.get(GRAPHIQL_PATH).unwrap(),
        )
    ));
    resp.set_content_type(mime::HTML);

    Ok(resp.into())
}



