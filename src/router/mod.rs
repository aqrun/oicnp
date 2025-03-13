use crate::{
    controllers::{stats, user, auth},
    middleware,
    models::{Tag, User},
};
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use zino::DefaultController;

pub fn routes() -> Vec<Router> {
    let mut list = Vec::new();

    let router = Router::new()
        .route("/auth/login", post(auth::login))
        .merge(
            Router::new()
                .route("/auth/refresh", get(auth::refresh))
                .route("/auth/logout", post(auth::logout))
                .layer(from_fn(middleware::init_user_session)),
        );
    list.push(router);

    let router = Router::new()
        .route("/", get(stats::index));
    list.push(router);

    let router = Router::new()
        .route("/user/new", post(user::new))
        .route("/user/{id}/delete", post(User::soft_delete))
        .route("/user/{id}/update", post(User::update))
        .route("/user/{id}/view", get(user::view))
        .route("/user/list", get(User::list))
        .route("/user/import", post(User::import))
        .route("/user/export", get(User::export))
        .route("/user/stats", get(user::stats));
    list.push(router);

    list
}

pub fn debug_routes() -> Vec<Router> {
    let mut routes = Vec::new();

    // Stats controller.
    let router = Router::new().route("/stats", get(stats::index));
    routes.push(router);

    // User schema controller.
    let router = Router::new()
        .route("/user/schema", get(User::schema))
        .route("/user/definition", get(User::definition))
        .route("/user/mock", get(User::mock));
    routes.push(router);

    // Tag schema controller.
    let router = Router::new()
        .route("/tag/schema", get(Tag::schema))
        .route("/tag/definition", get(Tag::definition))
        .route("/tag/mock", get(Tag::mock));
    routes.push(router);

    routes
}
