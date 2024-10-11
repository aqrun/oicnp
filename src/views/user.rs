use serde::{Deserialize, Serialize};
use oic_core::entities::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &UserModel) -> Self {
        Self {
            pid: user.uuid.to_string(),
            name: user.username.clone(),
            email: user.email.clone(),
        }
    }
}
