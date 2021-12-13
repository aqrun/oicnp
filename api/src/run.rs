use async_graphql::{Result};
use poem::{get, Route, Server,
           listener::TcpListener, EndpointExt,
};
use crate::services::G;
use crate::gql::{graphql, graphiql, build_schema};
use crate::typings::State;

pub async fn run() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let path = &G.config.graphql_url;
    let address = &G.config.host;
    let port = &G.config.port;

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
