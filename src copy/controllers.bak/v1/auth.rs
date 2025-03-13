use axum::debug_handler;
use loco_rs::prelude::*;
use oic_core::models::users::{LoginParams, RegisterParams};
use oic_core::typings::JsonRes;
use oic_core::utils::get_api_prefix;
use oic_core::services::{
    self,
    auth::{
        VerifyParams,
        ForgotParams,
        ResetParams,
        LoginResponse,
    },
};
use serde_json::Value;

// use crate::{
//     mailers::auth::AuthMailer,
// };

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> JsonRes<Value> {
    let res = services::auth::register(&ctx.db, params).await;
    JsonRes::from(res)
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyParams>,
) -> JsonRes<()> {
    let res = services::auth::verify(&ctx.db, params).await;
    JsonRes::from(res)
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> JsonRes<()> {
    let res = services::auth::forgot(&ctx.db, params).await;

    JsonRes::from(res)
    // AuthMailer::forgot_password(&ctx, &user).await?;
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> JsonRes<()> {
    let res = services::auth::reset(&ctx.db, params).await;

    JsonRes::from(res)
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> JsonRes<LoginResponse> {
    let res = services::auth::login(&ctx.db, &ctx.config, params).await;

    JsonRes::from(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix(get_api_prefix(super::VERSION, "auth").as_str())
        .add("/register", post(register))
        .add("/verify", post(verify))
        .add("/login", post(login))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
}
