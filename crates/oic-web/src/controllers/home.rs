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

pub fn routes() -> Router<AppContext> {
    Router::new()
        //  "/" 与所有路由冲突
        .route("/", get(index))
}