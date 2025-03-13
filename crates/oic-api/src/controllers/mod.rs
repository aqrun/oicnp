pub mod stats;

use axum::Router;

pub fn routes() -> Vec<Router> {
    let mut list = Vec::new();
    list.push(stats::routes());
    list
}