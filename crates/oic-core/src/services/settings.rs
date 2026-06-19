use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Settings {
    pub storage: StorageSettings,
    /// 正则路径
    pub user_agent_parser: String,
    /// 白名单接口
    pub public_apis: Vec<String>,
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
    /// OSS endpoint，如 https://oss-cn-hangzhou.aliyuncs.com
    pub endpoint: String,
    /// OSS AccessKey ID，建议通过环境变量注入
    pub access_key_id: String,
    /// OSS AccessKey Secret，建议通过环境变量注入
    pub access_key_secret: String,
    /// OSS region，如 cn-hangzhou
    pub region: String,
    /// OSS 对象前缀，如 uploads
    pub prefix: String,
}
