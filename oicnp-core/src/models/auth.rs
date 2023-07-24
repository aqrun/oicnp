use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub uid: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginInfo {
    pub token: String,
    pub uid: String,
    pub role: String,
    pub exp: usize,
}
