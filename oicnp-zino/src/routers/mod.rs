use axum::{
    Router,
    routing::{get, post},
};
// use zino::DefaultController;
use crate::controllers::user;

pub fn routes() -> Vec<Router> {
    let mut routes = Vec::new();

    let router = Router::new()
        .route("/user/list", get(user::list));

    routes.push(router);
    return routes;
}