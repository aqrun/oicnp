use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request, Response,
};
use oicnp_core::{
    G,
    typings::State,
};
use poem::{
    handler,
    web::{Data, Html, Json},
    IntoResponse,
};

#[handler]
pub async fn graphql(
    data: Data<&State>,
    gql_req: Json<Request>,
) -> Json<Response> {
    let mut gql_req = gql_req.0;
    let schema = data.0.schema.clone();

    // 将 poem 中生成请求上下文转入 graphql
    gql_req = gql_req.data(data.0.req_ctx.clone());

    Json(schema.execute(gql_req).await)
}

#[handler]
pub fn graphiql() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        &G.graphql_url,
    )))
}

