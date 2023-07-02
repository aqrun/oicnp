use crate::constants::{
    ENV_CONFIG_NAME,
};
use std::ffi::OsString;

/// 根据ENV获取配置文件路径
pub fn get_config_file_path() -> String {
    let env_file_path = std::env::var_os(ENV_CONFIG_NAME)
        .unwrap_or(OsString::from(""));
    env_file_path.into_string().unwrap_or(String::from(""))
}

pub fn get_env_config(cfg_name: &str) -> String {
    let cfg = std::env::var_os(cfg_name)
        .unwrap_or(OsString::from(""));
    cfg.into_string().unwrap_or(String::from(""))
}
