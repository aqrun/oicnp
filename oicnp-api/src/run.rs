use async_graphql::{Result};
use poem::{get, Route, Server,
           listener::TcpListener, EndpointExt,
};
use oicnp_core::G;
use crate::gql::{graphql, graphiql, build_schema};
use crate::typings::State;
use crate::utils::log;

pub async fn run() -> Result<(), std::io::Error> {
    log::init_log();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    // tracing_subscriber::fmt::init();

    let path = &G.graphql_url;
    let address = &G.host;
    let port = &G.port;

    let schema = build_schema().await;
    let state = State { schema };
    let app = Route::new()
        .at(path, get(graphiql).post(graphql))
        .data(state);

    println!("Playground: https://{}:{}", address, port);

    let listener = TcpListener::bind(
        format!("{}:{}", address, port));
    Server::new(listener)
        .run(app)
        .await
}
