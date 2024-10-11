use serde::{Deserialize, Serialize};
use oic_core::entities::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &UserModel, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.uuid.to_string(),
            name: user.username.clone(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}
