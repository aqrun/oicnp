use axum::http::{HeaderName, HeaderValue};
use oic::views::auth::UserSession;
use oic_core::entities::prelude::*;
use loco_rs::{app::AppContext, TestServer};

const USER_EMAIL: &str = "test@loco.com";
const USER_PASSWORD: &str = "1234";

pub struct LoggedInUser {
    pub user: UserModel,
    pub token: String,
}

pub async fn init_user_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let register_payload = serde_json::json!({
        "name": "loco",
        "email": USER_EMAIL,
        "password": USER_PASSWORD
    });

    //Creating a new user
    let _res = request.post("/auth/register").json(&register_payload).await;
    let user = UserModel::find_by_email(&ctx.db, USER_EMAIL)
        .await
        .unwrap();

    let verify_payload = serde_json::json!({
        "token": user.email_verify_token,
    });

    request.post("/auth/verify").json(&verify_payload).await;

    let response = request
        .post("/auth/login")
        .json(&serde_json::json!({
            "email": USER_EMAIL,
            "password": USER_PASSWORD
        }))
        .await;

    let session: UserSession = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user: UserModel::find_by_email(&ctx.db, USER_EMAIL)
            .await
            .unwrap(),
        token: session.token,
    }
}

pub fn auth_header(token: &str) -> (HeaderName, HeaderValue) {
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap();

    (HeaderName::from_static("authorization"), auth_header_value)
}
