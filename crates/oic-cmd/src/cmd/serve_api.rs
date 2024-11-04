use axum::{
    Router,
    routing::{get, post},
    response::{Html, Json},
};
use serde_json::{json, Value};

pub async fn run() {
    let app = Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
        .route("/", post(post_index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002").await.unwrap();
    println!("API 服务启动 {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn post_index() -> Json<Value> {
    Json(json!({
        "name": "alex",
    }))
}