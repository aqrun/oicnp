use super::{
    common,
    note,
    user,
};
use loco_rs::prelude::*;

pub fn routes() -> Vec<Routes> {
    let mut routes = Vec::new();
    routes.push(common::routes());
    routes.push(note::routes());
    routes.push(user::routes());
    routes
}
