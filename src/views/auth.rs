use serde::{Deserialize, Serialize};
use oic_core::entities::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub uid: String,
    pub uuid: String,
    pub username: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &UserModel, token: &String) -> Self {
        Self {
            token: token.to_string(),
            uid: user.uid.to_string(),
            uuid: user.uuid.to_string(),
            username: user.username.clone(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}
