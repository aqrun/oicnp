use serde::{Deserialize, Serialize};
use oic_core::entities::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSession {
    pub token: String,
    pub user: UserDetail,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UserDetail {
    pub pid: String,
    pub email: String,
    pub name: String,
    pub last_login: String,
}

impl UserSession {
    #[must_use]
    pub fn new(user: &UserModel, token: &String) -> Self {
        Self {
            token: token.to_string(),
            user: UserDetail {
                pid: user.uuid.to_string(),
                email: user.email.to_string(),
                name: user.username.to_string(),
                last_login: "n/a".to_string(),
            },
        }
    }
}
