use loco_rs::prelude::*;

pub mod v1;
pub mod home;

pub fn routes() -> Vec<Routes> {
    let mut routes = Vec::new();
    routes.push(home::routes());
    routes
}
