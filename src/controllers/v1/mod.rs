use loco_rs::prelude::*;
use oic_core::AppContext;

mod common;
mod note;
mod user;
mod node;
mod auth;
mod menu;
mod role;
mod permission;
mod tag;
mod category;
mod file;
mod cache;
mod online;
mod operation_log;
mod position;
mod department;

pub const VERSION: &str = "v1";

pub fn routes(_ctx: &AppContext) -> Vec<Routes> {
    let mut routes = Vec::new();
    routes.push(auth::routes());
    
    // 需要权限的路由列表
    let need_role_routes = vec![
        common::routes(),
        note::routes(),
        user::routes(),
        node::routes(),
        menu::routes(),
        role::routes(),
        permission::routes(),
        tag::routes(),
        category::routes(),
        file::routes(),
        cache::routes(),
        online::routes(),
        operation_log::routes(),
        position::routes(),
        department::routes(),
    ];
    routes.extend(need_role_routes);

    routes
}
