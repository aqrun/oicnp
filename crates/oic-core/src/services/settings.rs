use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Settings {
    pub storage: StorageSettings,
    /// 正则路径
    pub user_agent_parser: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct StorageSettings {
    /// 存储位置
    /// 可选值：local、oss、qiniu
    pub driver: String,
    /// 访问路径
    pub uri: String,
    /// 本地为磁盘存储位置
    /// oss 为 bucket 名称
    pub path: String,
}
