use axum::{
    Router,
    routing::{get, post},
    response::{Html, Json},
};
use serde_json::{json, Value};
use oic_core::AppContext;

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn post_index() -> Json<Value> {
    Json(json!({
        "name": "alex",
    }))
}

pub fn routes() -> Router<AppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
        .route("/", post(post_index))
}