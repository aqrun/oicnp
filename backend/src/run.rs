use async_graphql::{Result};
use tide::security::{CorsMiddleware, Origin};
use tide::http::headers::HeaderValue;
use crate::utils::G;
use crate::constants::{
    ADDRESS, PORT, GRAPHQL_PATH, GRAPHIQL_PATH,
};
use crate::gql::{graphql, graphiql, build_schema};
use crate::typings::State;

pub async fn run() -> Result<()> {
    tide::log::start();

    let schema = build_schema().await;
    let app_state = State { schema };
    let mut app = tide::with_state(app_state);

    app.at(G.get(GRAPHQL_PATH).unwrap())
        .post(graphql);
    app.at(G.get(GRAPHIQL_PATH).unwrap())
        .get(graphiql);

    let cors = CorsMiddleware::new()
        .allow_methods(
            "GET, POST, OPTIONS".parse::<HeaderValue>().unwrap()
        )
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    app.with(cors);
    app.listen(format!(
        "{}:{}",
        G.get(ADDRESS).unwrap(),
        G.get(PORT).unwrap(),
    ))
        .await?;

    Ok(())
}
