use crate::constants::{LOGIN_EXPIRE_TIME, LOGIN_REMEMBER_EXPIRE_TIME};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct ConsoleConfig {
    #[serde(rename(deserialize = "loginExpireTime", serialize = "loginExpireTime"))]
    pub login_expire_time: u64,
    #[serde(rename(deserialize = "loginRememberExpireTime", serialize = "loginRememberExpireTime"))]
    pub login_remember_expire_time: u64,
}

impl ConsoleConfig {
    pub fn new() -> Self {
        Self {
            login_expire_time: LOGIN_EXPIRE_TIME,
            login_remember_expire_time: LOGIN_REMEMBER_EXPIRE_TIME,
        }
    }
}
