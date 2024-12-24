use super::{
    common,
    note,
    user,
    node,
};
use loco_rs::prelude::*;

pub fn routes() -> Vec<Routes> {
    let mut routes = Vec::new();
    routes.push(common::routes());
    routes.push(note::routes());
    routes.push(user::routes());
    routes.push(node::routes());
    routes
}
