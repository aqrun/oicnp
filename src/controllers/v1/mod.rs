use loco_rs::prelude::*;
use oic_core::{AppContext, middleware::RoleRouteLayer};

mod common;
mod note;
mod user;
mod node;
mod auth;
mod menu;

pub const VERSION: &str = "v1";

pub fn routes(ctx: &AppContext) -> Vec<Routes> {
    let mut routes = Vec::new();
    routes.push(auth::routes());

    // 需要权限的路由列表
    let need_role_routes = vec![
        common::routes(),
        note::routes(),
        user::routes(),
        node::routes(),
        menu::routes()
    ];

    for router in need_role_routes.into_iter() {
        routes.push(add_auth_middleware(ctx, router));
    }

    routes
}

/// 需要检测授权的路由
fn add_auth_middleware(ctx: &AppContext, router: Routes) -> Routes {
    let router = router.layer(RoleRouteLayer::new(ctx.clone()));
    router
}
