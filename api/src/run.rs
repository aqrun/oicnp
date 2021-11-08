use async_graphql::{Result};
use poem::{get, Route, Server,
           listener::TcpListener, EndpointExt,
};
use crate::utils::G;
use crate::constants::{
    ADDRESS, PORT, GRAPHQL_PATH,
};
use crate::gql::{graphql, graphiql, build_schema};
use crate::typings::State;
use crate::dbs::get_connection_pool;
use std::sync::Arc;

pub async fn run() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let path = G.get(GRAPHQL_PATH).unwrap();
    let address = G.get(ADDRESS).unwrap();
    let port = G.get(PORT).unwrap();

    let schema = build_schema().await;
    let connection_pool = get_connection_pool();
    let arc_connection_pool = Arc::new(connection_pool);
    let state = State { schema, connection_pool: arc_connection_pool };
    let app = Route::new()
        .at(path, get(graphiql).post(graphql))
        .data(state);

    println!("Playground: https://{}:{}", address, port);

    let listener = TcpListener::bind(
        format!("{}:{}", address, port));
    let server = Server::new(listener).await?;
    server.run(app).await
}
