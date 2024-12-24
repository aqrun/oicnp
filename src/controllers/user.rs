use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::entities::prelude::*;
use oic_core::prelude::JWTWithUser;

use crate::views::user::CurrentResponse;

#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = UserModel::find_by_uuid(&ctx.db, &auth.claims.pid).await?;
    format::json(CurrentResponse::new(&user))
}

#[debug_handler]
async fn test1(auth: JWTWithUser<UserModel>, State(_ctx): State<AppContext>) -> Result<Response> {
    // let user = UserModel::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    format::json(auth)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/user")
        .add("/current", get(current))
        .add("/test1", get(test1))
}
