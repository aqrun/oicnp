use loco_rs::prelude::*;

mod common;
mod note;
mod user;
mod node;
mod auth;
mod menu;

pub const VERSION: &'static str = "v1";

pub fn routes() -> Vec<Routes> {
    let mut routes = Vec::new();
    routes.push(common::routes());
    routes.push(auth::routes());
    routes.push(note::routes());
    routes.push(user::routes());
    routes.push(node::routes());
    routes.push(menu::routes());
    routes
}
