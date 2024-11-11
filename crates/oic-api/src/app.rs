use axum::Router;
use anyhow::Result;
use tower_http::cors::{self, CorsLayer};
use crate::controllers;
use oic_core::app::{create_context, get_environment};

pub async fn run() -> Result<()> {
    let environment = get_environment();
    let app_ctx = create_context(&environment).await?;

    let app = Router::new()
        .nest("/", controllers::home::routes())
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_methods(cors::Any),
        )
        .with_state(app_ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002").await.unwrap();
    println!("API 服务启动 {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
