use async_graphql::{Result};
use poem::{get, handler, IntoResponse, Route, Server,
           listener::TcpListener, web::{Data}, EndpointExt,
};
use crate::utils::G;
use crate::constants::{
    ADDRESS, PORT, GRAPHQL_PATH, GRAPHIQL_PATH,
};
use crate::gql::{graphql, graphiql, build_schema};
use crate::typings::State;

pub async fn run() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let state = State { schema };
    let schema = build_schema().await;
    let app = Route::new()
        .at(G.get(GRAPHQL_PATH), get(graphiql).post(graphql))
        .data(state);

    let address = G.get(ADDRESS).unwrap();
    let port = G.get(PORT).unwrap();
    println!("Playground: https://{}:{}", address, port);

    let listener = TcpListener::bind(
        format!("{}:{}", address, port));
    let server = Server::new(listener).await?;
    server.run(app).await
}
