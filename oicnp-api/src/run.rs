use crate::gql::{build_schema, graphiql, graphql};
use crate::typings::State;
use crate::utils::log;
use crate::middleware::{AuthMiddleware, CtxMiddleware};
use async_graphql::Result;
use oicnp_core::{
    prelude::tokio::{self, time::Duration},
    G,
};
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};

pub async fn run() -> Result<(), std::io::Error> {
    log::init_log();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    let path = &G.graphql_url;
    let address = &G.host;
    let port = &G.port;

    let schema = build_schema().await;
    let state = State {
        schema,
        req_ctx: None
    };
    let app = Route::new()
        .at(path, get(graphiql).post(graphql))
        .with(AuthMiddleware)
        .with(CtxMiddleware)
        .data(state)
        ;

    println!("Playground: https://{}:{}", address, port);

    let listener = TcpListener::bind(format!("{}:{}", address, port));
    Server::new(listener)
        .run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await
}

